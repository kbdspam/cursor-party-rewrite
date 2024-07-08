// SPDX-License-Identifier: MIT
// Copyright 2023 Partykit, Inc.
// Copyright 2024 rtldg

// This file is effectively the client-side code of https://github.com/partykit/cursor-party
// but de-React-ed.

const cursorImage =
	"data:image/png;base64,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"
;

document.cursorPartyWs = document.cursorPartyWs || {
	socket: null,

	myid: 0,
	users: new Map(), // u[0] = untransformed_pos, u[1] = screen coordinates

	queueMyPositionTimeout: -1,
	updatePartyCountInterval: -1,

	myPositionWindow: [0, 0],
	myPositionDocument: [0, 0],

	windowDimensions: [0, 0],
	scrollDimensions: [0, 0],

	within: "", // "window" or "document"

	divsInitialized: false,
	shouldReconnect: false,
	reconnectTimeout: -1,

	updatedUsers: new Set(),

	reconnect: function() {
		if (!this.divsInitialized) this.initializeDivs();
		this.close();

		this.shouldReconnect = true;

		this.url = "wss://cursor-party-0.c.ookie.click/party/rock?_pk=0&from=cc";
		if (document.cursorPartyCC == "localhost" && false) {
			this.url = "ws://127.0.0.1:1999/party/rock?_pk=0&from=cc";
		}

		const socket = this.socket = new WebSocket(this.url);
		socket.addEventListener("open", (e) => {
			this.queueMyPosition(); // maybe the socket died, we reconnected, but we are also idle...
		});
		socket.addEventListener("close", this.socketCloseHandler = (e) => {
			this.handleClose();
			if (this.shouldReconnect) {
				this.reconnectTimeout = setTimeout(() => {
					if (this.shouldReconnect) {
						this.reconnect();
					}
				}, 3000);
			}
		});
		socket.addEventListener("message", this.socketMessageHandler = async (e) => {
			if (e.data instanceof Blob) {
				this.handleMessageBinary(await e.data.arrayBuffer());
			} else {
				this.handleMessageJSON(JSON.parse(e.data));
			}
		});
	},
	close: function() {
		this.shouldReconnect = false;
		clearTimeout(this.reconnectTimeout);
		this.reconnectTimeout = -1;
		if (this.socket != null) {
			this.socket.removeEventListener("close", this.socketCloseHandler);
			this.socket.removeEventListener("message", this.socketMessageHandler);
			this.socketCloseHandler({});
			// fucking socket doesn't want to close soon so we just unregister it and fuck off...
			this.socket.close();
			this.socket = null;
		}
	},
	initializeDivs: function() {
		// this.useCursorTracking("document");
		this.useCursorTracking("window");

		const canvas = this.canvas = document.createElement("canvas");
		document.body.appendChild(canvas);
		canvas.height = window.innerHeight;
		canvas.width = window.innerWidth;
		canvas.style.position = this.within == "window" ? "fixed" : "absolute";
		canvas.style.top = 0;
		canvas.style.left = 0;
		canvas.style.right = 0;
		canvas.style.bottom = 0;
		canvas.style.overflow = "clip";
		canvas.style.pointerEvents = "none";
		canvas.id = "cursorCanvas";
		// canvas.style.display = "block";
		this.canvas2dctx = canvas.getContext("2d");
		this.img = new Image(32, 32);
		this.img.src = cursorImage;

		this.updatePartyCountInterval = setInterval(() => {
			document.cursorPartyCount = this.users.size;
		}, 500);
		this.updateScrollDimensionsTimeout = setTimeout(() => {
			this.updateScrollDimensions();
		}, 100);

		this.divsInitialized = true;
	},
	handleClose: function() {
		// this.disableCursorTracking();
		clearTimeout(this.queueMyPositionTimeout);
		this.queueMyPositionTimeout = -1;
		// clearInterval(this.updatePartyCountInterval);
		// this.updatePartyCountInterval = -1;
		this.users.clear();
		redrawAllCursors();
	},
	useCursorTracking: function(within) {
		// within can be "window" or "document"
		this.within = within;

		window.addEventListener("resize", this.onResizeListener = (e) => this.onResize());
		this.onResize();
		window.addEventListener("mousemove", this.onMouseMoveListener = (e) => this.onMouseMove(e));
		window.addEventListener("touchmove", this.onTouchMoveListener = (e) => this.onMouseMove(e.touches[0]));
		window.addEventListener("scroll", this.onScrollListener = (e) => this.onScroll());
		// window.addEventListener("unload", this.onUnloadListener = (e) => this.onUnload(e));
		// window.addEventListener("onbeforeunload", this.onBeforeUnloadListener = (e) => this.onBeforeUnload(e));
	},
	disableCursorTracking: function() {
		window.removeEventListener("resize", this.onResizeListener);
		window.removeEventListener("mousemove", this.onMouseMoveListener);
		window.removeEventListener("touchmove", this.onTouchMoveListener);
		window.removeEventListener("scroll", this.onScrollListener);
	},
	updateScrollDimensions: function() {
		this.scrollDimensions = [document.documentElement.scrollWidth, document.documentElement.scrollHeight];
		this.transformAllCursors();
	},
	onResize: function() {
		this.updateScrollDimensions();
		this.windowDimensions = [window.innerWidth, window.innerHeight];
		if (this.canvas) {
			this.canvas.height = window.innerHeight;
			this.canvas.width = window.innerWidth;
		}
		this.transformAllCursors();
	},
	onScroll: function() {
		if (this.within != "document") return;
		this.myPositionDocument = [
			this.myPositionWindow[0] + (window.scrollX || document.documentElement.scrollLeft),
			this.myPositionWindow[1] + (window.scrollY || document.documentElement.scrollTop),
		];
		this.transformAllCursors();
	},
	onMouseMove: function(e) {
		this.myPositionWindow = [e.clientX, e.clientY];
		if (this.within == "document")
			this.myPositionDocument = [e.pageX, e.pageY];
		this.queueMyPosition();
	},
	onUnload: function(e) {
		this.close();
		console.log("onUnload");
	},
	onBeforeUnload: function(e) {
		this.close();
		console.log("onBeforeUnload");
	},
	transformAllCursors: function() {
		// - transform all cursors from users
		this.users.forEach((v, k, m) => v[1] = this.screenCoordinates(v[0]));
		this.redrawAllCursors();
	},
	redrawAllCursors: function() {
		if (!this.canvas)
			return;
		this.canvas2dctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
		const users = [...this.users.keys()].sort().reverse();
		users.map((id) => this.users.get(id)).forEach((u) => {
			this.canvas2dctx.drawImage(this.img, u[1][0], u[1][1], 32, 32);
		});
	},
	screenCoordinates: function(cur) {
		const bounds = this.within == "window" ? this.windowDimensions : this.scrollDimensions;
		// round them because apparently canvas elements like whole numbers
		return [Math.round(cur[0] * bounds[0]), Math.round(cur[1] * bounds[1])];
	},
	queueMyPosition: function() {
		if (this.socket != null && this.socket.readyState == 1 && this.queueMyPositionTimeout == -1) {
			this.queueMyPositionTimeout = setTimeout(() => {
				if (this.socket == null) return;
				this.queueMyPositionTimeout = -1;
				const bounds = this.within == "window" ? this.windowDimensions : this.scrollDimensions;
				const cur = this.within == "window" ? this.myPositionWindow : this.myPositionDocument;
				const buffer = new ArrayBuffer(8);
				const view = new DataView(buffer);
				view.setFloat32(0, cur[0] / bounds[0], true); // little-endian float...
				view.setFloat32(4, cur[1] / bounds[1], true); // little-endian float...
				this.socket.send(buffer);
			}, 25);
		}
	},
	handleMessageBinary: function(buffer) {
		const updatedUsers = new Map();
		const removedUsers = new Set();
		const redrawTheseUsers = new Set();

		const view = new DataView(buffer);
		for (let pos = 0, _c = view.byteLength / 4; pos < _c;) {
			// we read all these values as little-endian
			const type = view.getUint32(pos++ * 4, true);
			const count = view.getUint32(pos++ * 4, true);
			if (!count) continue;
			if (type == 1 || type == 2) {
				// add || presence
				for (let i = 0; i < count; i++) {
					const id = view.getUint32(pos++ * 4, true);
					const x = view.getFloat32(pos++ * 4, true);
					const y = view.getFloat32(pos++ * 4, true);
					if (id == this.myid)
						continue;
					redrawTheseUsers.add(id);
					updatedUsers.set(id, [[x, y], this.screenCoordinates([x, y])]);
				}
			} else if (type == 3) {
				for (let i = 0; i < count; i++) {
					const id = view.getUint32(pos++ * 4, true);
					removedUsers.add(id);
				}
			}
		}

		const idleUsers = new Map(
			(new Set(this.users.keys()))
			.difference(
				redrawTheseUsers.union(removedUsers)
			)
			.entries()
			.map(([id, _id]) => [id, this.users.get(id)])
			.filter((v) => v[1])
		);
		// console.log("idleUsers", idleUsers);

		const clearThesePositions = [];

		this.doOverlapCheck = (cu) => {
			if (!cu) return;
			clearThesePositions.push(cu);
			idleUsers.forEach((idle, id, m) => {
				const noOverlap = (cu[0])      > (idle[1][0]+32) ||
				                  (idle[1][0]) > (cu[0]+32)      ||
				                  (cu[1])      > (idle[1][1]+32) ||
				                  (idle[1][1]) > (cu[1]+32);
				/*
				if (   (cu[0])    < (idle[1][0]+32) // RectA.Left   < RectB.Right
					&& (cu[0]+32) > (idle[1][0])    // RectA.Right  > RectB.Left
					&& (cu[1])    > (idle[1][1]+32) // RectA.Top    > RectB.Bottom
					&& (cu[1]+32) < (idle[1][1]))   // RectA.Bottom < RectB.Top
				*/
				if (!noOverlap)
				{
					// console.log(`overlapping ${id}`);
					idleUsers.delete(id);
					redrawTheseUsers.add(id);
					this.doOverlapCheck(idle[1]);
				}
			});
		};

		removedUsers.forEach((v, k, s) => {
			const u = this.users.get(k);
			if (u) {
				this.users.delete(k);
				this.doOverlapCheck(u[1]);
			}
		});
		updatedUsers.forEach((v, k, m) => {
			const u = this.users.get(k);
			if (u) {
				this.doOverlapCheck(u[1]);
			}
			this.users.set(k, v);
			this.doOverlapCheck(v[1]);
		});
		this.doOverlapCheck = null;

		clearThesePositions.forEach((v, k, a) => this.canvas2dctx.clearRect(v[0], v[1], 32, 32));
		// console.log("redraw these", redrawTheseUsers);
		[...redrawTheseUsers].sort().reverse().forEach((id, k, a) => {
			const u = this.users.get(id);
			if (u)
				this.canvas2dctx.drawImage(this.img, u[1][0], u[1][1], 32, 32);
			else
				console.log(`${id} was bad?`);
		});
	},
	handleMessageJSON: function(msg) {
		if (msg.myid) {
			this.myid = msg.myid;
		} else if (msg.heartbeat) {
			this.socket.send(`{"heartbeat":true}`);
		}
	},
};
document.cursorPartyWs.reconnect();
