// SPDX-License-Identifier:
// Copyright

#![forbid(unsafe_code)]
#![feature(let_chains)]

use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::{buffer, protocol::WebSocketConfig};

use futures::{SinkExt, StreamExt};
use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, Mutex},
	time::Duration,
};
use thiserror::Error;

// Cookie Clicker runs at 30 fps so there's no reason to go higher...
const FPS: f64 = 30.0;
const BROADCAST_INTERVAL_SECS: f64 = 1.0 / FPS;

type ID = u32; // Don't change this willy-nilly because our update message assumes 32-bit integers.
#[repr(u32)]
enum BinaryType {
	//Add = 1, // Unused since Update is exactly the same...
	Update = 2,
	Remove = 3,
}

struct AbortTaskOnDrop<T>(tokio::task::JoinHandle<T>);
impl<T> Drop for AbortTaskOnDrop<T> {
	fn drop(&mut self) {
		self.0.abort();
	}
}

#[derive(Serialize, Deserialize, Debug)]
enum JsonMsg {
	#[allow(non_camel_case_types)]
	myid(u32),
}

#[derive(Copy, Clone)]
struct Position {
	x: f32,
	y: f32,
}
#[derive(Error, Debug)]
enum PositionError {
	#[error("Bad message size (message size should be 8 bytes)")]
	BadMessageSize,
	#[error("Position floats are outside of range 0.0 <= float <= 1.0")]
	OutOfBoundsFloats,
}
impl Position {
	fn get(b: &[u8]) -> Result<Self, PositionError> {
		if b.len() == 8 {
			let x = f32::from_le_bytes((&b[0..4]).try_into().unwrap());
			let y = f32::from_le_bytes((&b[4..8]).try_into().unwrap());
			if x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0 {
				Ok(Position { x, y })
			} else {
				Err(PositionError::OutOfBoundsFloats)
			}
		} else {
			Err(PositionError::BadMessageSize)
		}
	}
}
struct User {
	sender: tokio::sync::mpsc::UnboundedSender<tokio_tungstenite::tungstenite::Message>,
	pos: Position,
}
enum BroadcastMessage {
	Setup(SharedState),
	Queue,
}
struct State {
	users: HashMap<ID, User>,
	// queued_adds: Vec<ID>,
	queued_removes: Vec<ID>,
	queued_updates: HashMap<ID, Position>,
	broadcast_task: tokio::task::JoinHandle<()>,
	broadcaster: tokio::sync::mpsc::UnboundedSender<BroadcastMessage>,
}
struct SharedState(Arc<Mutex<State>>);
impl std::ops::Deref for SharedState {
	type Target = Arc<Mutex<State>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Clone for SharedState {
	fn clone(&self) -> Self {
		SharedState(self.0.clone())
	}
}
impl Default for SharedState {
	fn default() -> Self {
		let state = SharedState(Arc::new(Mutex::new(State::default())));
		state.lock().unwrap().setup(state.clone());
		state
	}
}
// type SharedState = Arc<Mutex<State>>;
impl Default for State {
	fn default() -> Self {
		let (s, r) = tokio::sync::mpsc::unbounded_channel::<BroadcastMessage>();
		State {
			users: Default::default(),
			queued_removes: Default::default(),
			queued_updates: Default::default(),
			broadcast_task: tokio::spawn(State::broadcaster_thread(r)),
			broadcaster: s,
		}
	}
}
impl State {
	fn setup(&mut self, ss: SharedState) {
		// this kind of sucks
		let _ = self.broadcaster.send(BroadcastMessage::Setup(ss));
	}
	fn queue_remove(&mut self, id: ID) {
		if let Some(_) = self.users.remove(&id) {
			let _ = self.queued_updates.remove(&id);
			self.queued_removes.push(id);
			self.queue_broadcast();
		}
	}
	fn queue_update(&mut self, id: ID, pos: Position) {
		let _ = self.queued_updates.insert(id, pos);
		self.users.get_mut(&id).unwrap().pos = pos;
		self.queue_broadcast();
	}
	fn queue_broadcast(&mut self) {
		let _ = self.broadcaster.send(BroadcastMessage::Queue);
	}
	fn broadcast_cursors(&mut self) {
		let all_users: HashSet<ID> = self.users.keys().cloned().collect();
		let updated_users: HashSet<ID> = self.queued_updates.keys().cloned().collect();
		let idle_users: HashSet<ID> = all_users.difference(&updated_users).cloned().collect();

		if idle_users.len() > 0 {
			let msg = self.make_msg_for_idle_users();
			for id in &idle_users {
				// TODO: Properly remove shit if shit happens here...
				let _ = self
					.users
					.get(id)
					.unwrap()
					.sender
					.send(tokio_tungstenite::tungstenite::Message::Binary(msg.clone()));
				// TODO: remove .clone() once this has been migrated from tungstenite to actix...
			}
		}

		if updated_users.len() > 0 {
			for id in &updated_users {
				if let Some(msg) = self.make_msg_for_updated_user(*id) {
					// TODO: Properly remove shit if shit happens here...
					let _ = self
						.users
						.get(id)
						.unwrap()
						.sender
						.send(tokio_tungstenite::tungstenite::Message::Binary(msg));
				}
			}
		}

		self.queued_updates.clear();
		self.queued_removes.clear();
	}
	fn make_msg_for_updated_user(&mut self, skip_this_id: ID) -> Option<Vec<u8>> {
		let mut msg = None;
		// if .len() < 1 then this function wasn't called
		// if .len() == 1 then this user is the only user in the queued updates...
		if self.queued_updates.len() > 1 {
			let msg = msg.get_or_insert(vec![]);
			msg.extend_from_slice(&((BinaryType::Update as u32).to_le_bytes()));
			msg.extend_from_slice(&(((self.queued_updates.len() - 1) as u32).to_le_bytes()));
			for (iter_id, pos) in &self.queued_updates {
				if *iter_id == skip_this_id {
					continue;
				}
				msg.extend_from_slice(&iter_id.to_le_bytes());
				msg.extend_from_slice(&pos.x.to_le_bytes());
				msg.extend_from_slice(&pos.y.to_le_bytes());
			}
		}
		// send ALL removes...
		if self.queued_removes.len() > 0 {
			let msg = msg.get_or_insert_with(|| vec![]);
			msg.extend_from_slice(&((BinaryType::Update as u32).to_le_bytes()));
			msg.extend_from_slice(&((self.queued_removes.len() as u32).to_le_bytes()));
			for id in &self.queued_removes {
				msg.extend_from_slice(&id.to_le_bytes());
			}
		}
		msg
	}
	fn make_msg_for_idle_users(&mut self) -> Vec<u8> {
		let updates_len = if self.queued_updates.len() > 0 {
			2 + self.queued_updates.len()
		} else {
			0
		};
		let removes_len = if self.queued_removes.len() > 0 {
			2 + self.queued_removes.len()
		} else {
			0
		};
		let buffer_size = (updates_len + removes_len) * 4;
		assert!(buffer_size > 0); // ?
		let mut msg = vec![];
		msg.reserve_exact(buffer_size);
		if self.queued_updates.len() > 0 {
			msg.extend_from_slice(&((BinaryType::Update as u32).to_le_bytes()));
			msg.extend_from_slice(&((self.queued_updates.len() as u32).to_le_bytes()));
		}
		if self.queued_removes.len() > 0 {
			msg.extend_from_slice(&((BinaryType::Remove as u32).to_le_bytes()));
			msg.extend_from_slice(&((self.queued_removes.len() as u32).to_le_bytes()));
		}
		msg
	}
	fn connect_user(&mut self, id: ID, new_user: User) -> anyhow::Result<()> {
		let len = self.users.len();
		if len > 0 {
			let mut msg: Vec<u8> = vec![];
			msg.reserve_exact((2 + (len * 3)) * 4);
			msg.extend_from_slice(&((BinaryType::Update as u32).to_be_bytes()));
			msg.extend_from_slice(&((self.users.len() as u32).to_be_bytes()));
			for (id, user) in &self.users {
				msg.extend_from_slice(&id.to_le_bytes());
				msg.extend_from_slice(&user.pos.x.to_le_bytes());
				msg.extend_from_slice(&user.pos.y.to_le_bytes());
			}
			new_user
				.sender
				.send(tokio_tungstenite::tungstenite::Message::Binary(msg))?;
		}
		assert!(self.users.insert(id, new_user).is_none());
		Ok(())
	}
	async fn broadcaster_thread(mut r: tokio::sync::mpsc::UnboundedReceiver<BroadcastMessage>) {
		let mut state = None;
		while let Some(msg) = r.recv().await {
			match msg {
				BroadcastMessage::Setup(ss) => state = Some(ss),
				BroadcastMessage::Queue => {
					tokio::time::sleep(Duration::from_secs_f64(BROADCAST_INTERVAL_SECS)).await;
					loop {
						match r.try_recv() {
							// Got a Queue, ignore.
							Ok(_) => (),
							// Drained Queue's.
							Err(tokio::sync::mpsc::error::TryRecvError::Empty) => break,
							// Womp womp.
							Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => return,
						}
					}
					state.as_mut().unwrap().lock().unwrap().broadcast_cursors();
				}
			}
		}
	}
}

async fn handle_websocket(stream: tokio::net::TcpStream, id: ID, state: &SharedState) -> anyhow::Result<()> {
	let ws = tokio_tungstenite::accept_async_with_config(
		stream,
		Some(WebSocketConfig {
			accept_unmasked_frames: false,
			max_message_size: Some(1024),
			max_frame_size: Some(1024),
			..Default::default()
		}),
	)
	.await?;

	let (mut ws_s, mut ws_r) = ws.split();
	ws_s.send(tokio_tungstenite::tungstenite::Message::Text(serde_json::to_string(
		&JsonMsg::myid(id),
	)?))
	.await?;

	let (ch_s, mut ch_r) = tokio::sync::mpsc::unbounded_channel::<tokio_tungstenite::tungstenite::Message>();

	// heartbeat which can probably be disabled...
	// also not very cancellable because the sleep...
	let _heartbeat_task = AbortTaskOnDrop(tokio::spawn({
		let ch_s = ch_s.clone();
		async move {
			loop {
				tokio::time::sleep(Duration::from_secs(33)).await;
				if ch_s
					.send(tokio_tungstenite::tungstenite::Message::Ping(vec![b'6', b'9']))
					.is_err()
				{
					break;
				}
			}
		}
	}));
	let _relay_task = AbortTaskOnDrop(tokio::spawn(async move {
		while let Some(msg) = ch_r.recv().await
			&& ws_s.send(msg).await.is_ok()
		{}
	}));

	state.lock().unwrap().connect_user(
		id,
		User {
			sender: ch_s.clone(),
			pos: Position { x: 0.0, y: 0.0 },
		},
	)?;

	while let Some(msg) = ws_r.next().await {
		let msg = msg?;
		match msg {
			tokio_tungstenite::tungstenite::Message::Text(s) => {
				println!("s = '{s}'");
			}
			tokio_tungstenite::tungstenite::Message::Binary(b) => {
				let pos = Position::get(&b)?;
				state.lock().unwrap().queue_update(id, pos);
			}
			_ => {}
		}
	}

	Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:1999".to_string());
	let listener = tokio::net::TcpListener::bind(addr).await?;
	let mut latest_id = 0u32;
	let state: SharedState = Default::default();
	loop {
		if let Ok((stream, addr)) = listener.accept().await {
			latest_id = latest_id.checked_add(1).unwrap(); // just fucking die if we hit 4 billion users...
			let id = latest_id;
			let state = state.clone();
			println!("accepted client {id} {addr}");
			tokio::spawn(async move {
				let _ = handle_websocket(stream, id, &state).await;
				state.lock().unwrap().queue_remove(id);
				println!("finished with client {id} {addr}");
			});
		}
	}
}
