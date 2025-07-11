/**
 * Metra JS shell
 * Made with Love (care) and Hate (web development)
 * (c) addie.sh 2025
 */

/**
 * @typedef {Object} MetraExports
 * @prop {Object} exports
 * @prop {WebAssembly.Memory} exports.memory
 * @prop {WebAssembly.Global} exports.metraVarBigEndian
 * @prop {() => 0|1} exports.metraUpdate
 * @prop {() => void} exports.metraMain
 * @prop {() => void} exports.metraClean
 */

/**
 * @typedef {WebAssembly.Instance & MetraExports} Metra
 */

/**
 * @typedef {Object} MetraResourceTexture
 * @prop {"texture"} resourceType
 * @prop {HTMLImageElement} data
 */

/**
 * @typedef {Object} MetraResourceSound
 * @prop {"sound"} resourceType
 * @prop {HTMLAudioElement} data
 */

/**
 * @typedef {MetraResourceTexture|MetraResourceSound} MetraResource
 * @prop {string} checksum A precomputed SHA-256 hash of the resource, to be compared against any manifest or source.
 */

/**
 * @typedef {Object} MetraResourceManifestEntry
 * @prop {"texture"|"sound"} resourceType The type of resource.
 * TODO: add width/height hints to texture
 * @prop {string} path A path to the resource.
 * @prop {string} checksum A precomputed SHA-256 hash of the resource, to be compared against any manifest or source.
 */


// noinspection SpellCheckingInspection
console.log(
	"%c   \n%cMade with Metra",
	// TODO: optimize the project-wide logo instead of creating a subset
	"font-size:128px;background:local url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgNSAzMiAyMiI+PGRlZnM+PGxpbmVhckdyYWRpZW50IGlkPSJlIj48c3RvcCBvZmZzZXQ9IjAiIHN0eWxlPSJzdG9wLWNvbG9yOiMwYzE1MjEiLz48c3RvcCBvZmZzZXQ9IjEiIHN0eWxlPSJzdG9wLWNvbG9yOiMzNTNjNTgiLz48L2xpbmVhckdyYWRpZW50PjxsaW5lYXJHcmFkaWVudCBpZD0iZiI+PHN0b3Agb2Zmc2V0PSIwIiBzdHlsZT0ic3RvcC1jb2xvcjojZmZmO3N0b3Atb3BhY2l0eTouNyIvPjxzdG9wIG9mZnNldD0iMSIgc3R5bGU9InN0b3AtY29sb3I6I2ZmZjtzdG9wLW9wYWNpdHk6MCIvPjwvbGluZWFyR3JhZGllbnQ+PGxpbmVhckdyYWRpZW50IGlkPSJkIj48c3RvcCBvZmZzZXQ9IjAiIHN0eWxlPSJzdG9wLWNvbG9yOiMwZWZmMGUiLz48c3RvcCBvZmZzZXQ9IjEiIHN0eWxlPSJzdG9wLWNvbG9yOiM4MmZmODIiLz48L2xpbmVhckdyYWRpZW50PjxsaW5lYXJHcmFkaWVudCBpZD0iYyI+PHN0b3Agb2Zmc2V0PSIwIiBzdHlsZT0ic3RvcC1jb2xvcjojZmZjYjAwIi8+PHN0b3Agb2Zmc2V0PSIxIiBzdHlsZT0ic3RvcC1jb2xvcjojZmZlNTc1Ii8+PC9saW5lYXJHcmFkaWVudD48bGluZWFyR3JhZGllbnQgaWQ9ImIiPjxzdG9wIG9mZnNldD0iMCIgc3R5bGU9InN0b3AtY29sb3I6I2ZmMjYyNiIvPjxzdG9wIG9mZnNldD0iMSIgc3R5bGU9InN0b3AtY29sb3I6I2ZmNWM1YyIvPjwvbGluZWFyR3JhZGllbnQ+PGxpbmVhckdyYWRpZW50IGlkPSJhIj48c3RvcCBvZmZzZXQ9IjAiIHN0eWxlPSJzdG9wLWNvbG9yOiM1MjZhY2MiLz48c3RvcCBvZmZzZXQ9IjEiIHN0eWxlPSJzdG9wLWNvbG9yOiNlZGYwZmEiLz48L2xpbmVhckdyYWRpZW50PjxsaW5lYXJHcmFkaWVudCBpZD0iZyIgeDE9IjE2LjUiIHgyPSIyMC41IiB5MT0iMjciIHkyPSIzIiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgaHJlZj0iI2EiLz48bGluZWFyR3JhZGllbnQgaWQ9Im4iIHgxPSI3IiB4Mj0iOC41IiB5MT0iMjQiIHkyPSIxOCIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiIGhyZWY9IiNiIi8+PGxpbmVhckdyYWRpZW50IGlkPSJvIiB4MT0iMTciIHgyPSIxOC41IiB5MT0iMjQiIHkyPSIxOCIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiIGhyZWY9IiNjIi8+PGxpbmVhckdyYWRpZW50IGlkPSJwIiB4MT0iMjUiIHgyPSIyNi41IiB5MT0iMjQiIHkyPSIxOCIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiIGhyZWY9IiNkIi8+PGxpbmVhckdyYWRpZW50IGlkPSJoIiB4MT0iMjQuNSIgeDI9IjEyLjUiIHkxPSIyNSIgeTI9IjciIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBocmVmPSIjZSIvPjxsaW5lYXJHcmFkaWVudCBpZD0ibCIgeDE9IjI0LjUiIHgyPSIxMi41IiB5MT0iMjUiIHkyPSI3IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgaHJlZj0iI2UiLz48bGluZWFyR3JhZGllbnQgaWQ9ImsiIHgxPSIyNC41IiB4Mj0iMTIuNSIgeTE9IjI1IiB5Mj0iNyIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiIGhyZWY9IiNlIi8+PGxpbmVhckdyYWRpZW50IGlkPSJqIiB4MT0iMjQuNSIgeDI9IjEyLjUiIHkxPSIyNSIgeTI9IjciIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBocmVmPSIjZSIvPjxyYWRpYWxHcmFkaWVudCBpZD0iaSIgY3g9IjguNzkxIiBjeT0iNi4wNDgiIHI9IjEzLjY3NCIgZng9IjguNzkxIiBmeT0iNi4wNDgiIGdyYWRpZW50VHJhbnNmb3JtPSJ0cmFuc2xhdGUoLTEuNzg2IDEuOTI0KXNjYWxlKDEuMTcwMDgpIiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgaHJlZj0iI2YiLz48L2RlZnM+PHBhdGggZD0ibTMgMjQgMy41LTE0TDI1IDZjNS0xLjA4MSA2IDIgNSA2bC0zIDEyaC00bDItOGMxLTQtMy00LTQgMGwtMiA4aC00bDItOGMxLTQtMy00LTQgMGwtMiA4eiIgc3R5bGU9ImZpbGw6dXJsKCNnKTtzdHJva2U6dXJsKCNoKTtzdHJva2Utd2lkdGg6MjtwYWludC1vcmRlcjpzdHJva2UgZmlsbCBtYXJrZXJzIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtLjUgMSkiLz48cGF0aCBkPSJtMyAyNCAzLjUtMTRMMjUgNmM1LTEuMDgxIDYgMiA1IDZsLTMgMTJoLTRsMi04YzEtNC0zLTQtNCAwbC0yIDhoLTRsMi04YzEtNC0zLTQtNCAwbC0yIDh6IiBzdHlsZT0iZmlsbDp1cmwoI2kpIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtLjUgMSkiLz48cGF0aCBkPSJtNCAxOCAuNS0xaDlsLS41IDFaIiBzdHlsZT0iZmlsbDp1cmwoI2opIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtLjUgMSkiLz48cGF0aCBkPSJtMTYgMTggLjUtMWg1bC0uNSAxWiIgc3R5bGU9ImZpbGw6dXJsKCNrKSIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoLS41IDEpIi8+PHBhdGggZD0ibTI0IDE4IC41LTFoNWwtLjUgMVoiIHN0eWxlPSJmaWxsOnVybCgjbCkiIHRyYW5zZm9ybT0idHJhbnNsYXRlKC0uNSAxKSIvPjxwYXRoIGQ9Im0zIDI0IDEuNS02aDhMMTEgMjRaIiBzdHlsZT0iZmlsbDp1cmwoI24pIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtLjUgMSkiLz48cGF0aCBkPSJNMTUgMjRoNGwxLjUtNmgtNHoiIHN0eWxlPSJmaWxsOnVybCgjbykiIHRyYW5zZm9ybT0idHJhbnNsYXRlKC0uNSAxKSIvPjxwYXRoIGQ9Ik0yNC41IDE4aDRMMjcgMjRoLTR6IiBzdHlsZT0iZmlsbDp1cmwoI3ApIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtLjUgMSkiLz48L3N2Zz4=') left/contain no-repeat;",
	"color:light-dark(#000,#fff);font-size:32px;font-family:Inter Display,Inter,system-ui;",
);
console.log(
	"%c\u00A9 addie.sh",
	"color:light-dark(#000,#fff);font-size:16px;font-family:Inter Display,Inter,system-ui;"
);

{
	// https://bsky.app/profile/addie.sh/post/3lqq6ixhjp22q
	let u16 = new Uint16Array([0xACAB]);
	let u8 = new Uint8Array(u16.buffer);

	switch (u8[0]) {
		case 0xAC: {
			throw Error("Big-endian systems are not supported!");
		}
		case 0xAB: {
			console.info("Running on little-endian system, no corrections required");
			break;
		}
		default: {
			// :(
			throw Error(`Eldritch endianness (0x${u8[0].toString(16)}${u8[1].toString(16)})`);
		}
	}
}

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById('metra');
/** @type {WebGL2RenderingContext} */
const gl = canvas.getContext('webgl2');


const context = {
	/** @type {Metra} */
	metra: undefined,

	// TODO: this is slow, replace with something faster.
	/** @type {Map<number, WebGLBuffer|WebGLProgram|WebGLShader|WebGLFramebuffer|WebGLRenderbuffer|WebGLTexture|WebGLVertexArrayObject>} */
	objects: new Map(),
};

/**
 * this is a horrible """algorithm""" and totally sucks.
 * but I'm tired, and we have a massive performance budget
 * @returns {number}
 */
function findKey() {
	let key = context.objects.size + 1;
	while (true) {
		if (context.objects.get(key) === undefined) {
			return key;
		} else {
			key = (key + 1) % 0xFFFFFFFF;
		}
	}
}

/** @type {?Uint8Array} */
let cachedSaveBuffer = null;
/** @type {?string} */
let cachedSaveString = null;
// note to self: JS ignores null bytes
let textDecoder = new TextDecoder();
let textEncoder = new TextEncoder();

// These variables are for engine-wide resources.
/** @type {Record<string, MetraResource>} */
let resourceBank = {};

// TODO: get a resource manifest from somewhere, either embedded in the binary or exported to a separate file
/** @type {Record<string, MetraResourceManifestEntry>} */
let resourceManifest = {
	"antonymph": {
		resourceType: 'sound',
		path: "resources/antonymph.wav",
		checksum: "c44c33dc1fd8c50591cf6544c0b1aa8bf09454c7fc197334e69d0ac80d6df9c9"
	},
	"noise": {
		resourceType: 'texture',
		path: "resources/noise.png",
		checksum: "7e8f8d20fea0d8645e77cb4b43678d3257c91e0ab09c6cd9e14e6d6b3676c6a1"
	},
	"rust": {
		resourceType: 'texture',
		path: "resources/rust.gif",
		checksum: "1ef8b040bbc80b5a741afe429ae3110af3cbdc8ef5b7726ca64d2d6f0afd0cf5"
	}
};

// noinspection JSUnusedGlobalSymbols
const importObject = {
	metraSys: {
		/**
		 * @param {1|2|3|4|5|6} level 
		 * @param {number} _targetPtr 
		 * @param {number} _targetLen 
		 * @param {number} locationPtr
		 * @param {number} locationLen
		 * @param {number} contentPtr 
		 * @param {number} contentLen 
		 */
		log: function (
			level,
			_targetPtr,
			_targetLen,
			locationPtr,
			locationLen,
			contentPtr,
			contentLen,
		) {
			let levelFnMap = ['error', 'warn', 'info', 'debug', 'trace', 'error'];
			let levelStringMap = ['ERROR', 'WARNING', 'INFO', 'DEBUG', 'TRACE', 'PANIC!'];
			// currently unused
			// let target = ((targetPtr && targetLen) ? textDecoder.decode(
			// 	context.metra.exports.memory.buffer.slice(targetPtr, targetPtr + targetLen),
			// ) : "");
			let location = ((locationPtr && locationLen) ? textDecoder.decode(
				context.metra.exports.memory.buffer.slice(locationPtr, locationPtr + locationLen),
			) : "");
			let content = ((contentPtr && contentLen) ? textDecoder.decode(
				context.metra.exports.memory.buffer.slice(contentPtr, contentPtr + contentLen),
			).replaceAll('%', '%%') : "");
			let string = `%c[${levelStringMap[level - 1]} ${location}]%c ${content}`;
			console[levelFnMap[level - 1]](
				string,
				"font-weight:800;",
				""
			);
		},

		/**
		 * @return {number} The time (in milliseconds) since program start.
		 */
		getTime: function () {
			// TODO: stub...?
			return performance.now();
		},

		/**
		 * @return {number}
		 */
		getRandom: function () {
			return Math.random();
		},

		/**
		 * @param {number} dataPtr The pointer to write to
		 * @param {number} dataLen
		 * @return {number} 0 on success, length required if the buffer is null/too small, u32::MAX on unknown error
		 */
		loadPersistent: function (
			dataPtr,
			// TODO: technically dataLen probably isn't necessary,
			//		 but we don't have safeguards in place if the data changes mid-transfer.
			dataLen,
		) {
			if (cachedSaveBuffer === null) {
				try {
					cachedSaveString = localStorage.getItem('metraPersistent');
					if (cachedSaveString == null) {
						cachedSaveBuffer = new Uint8Array(0);
					} else {
						cachedSaveBuffer = textEncoder.encode(cachedSaveString);
					}
				} catch (err) {
					console.error("Error reading from persistent data:", err);
					return 0xFFFFFFFF;
				}
			}

			if (dataPtr === 0 || dataLen === 0) {
				return cachedSaveBuffer.byteLength;
			} else {
				// this branch should never be called if len == 0, as enforced by caller
				let view = new Uint8Array(context.metra.exports.memory.buffer, dataPtr, dataLen);
				view.set(cachedSaveBuffer, 0);
				return 0;
			}
		},

		/**
		 * @param {number} dataPtr 
		 * @param {number} dataLen 
		 * @returns {0|1} 1 on success or 0 otherwise.
		 */
		savePersistent: function (
			dataPtr,
			dataLen,
		) {
			try {
				let buf = context.metra.exports.memory.buffer.slice(dataPtr, dataPtr + dataLen);
				cachedSaveString = textDecoder.decode(buf);
				localStorage.setItem('metraPersistent', cachedSaveString);
				cachedSaveBuffer = new Uint8Array(buf);
				return 1;
			} catch (err) {
				console.error("Error writing to persistent data:", err);
				return 0;
			}
		},

		createVertexArray: function () {
			console.warn("metraSys.createVertexArray: stub");
			gl.createVertexArray();
			// TODO:
			return 0xACAB;
		},

		/**
		 * @param {number} vertexArray
		 * @returns {0|1}
		 */
		dropVertexArray: function (vertexArray) {
			let obj = context.objects.get(vertexArray);
			if (obj === undefined) {
				return 0;
			} else {
				gl.deleteVertexArray(context.objects[vertexArray]);
				// context.freeBuffers.push()
				context.objects.delete(vertexArray);
				console.debug(`dropped buffer with ID ${vertexArray}`);
				return 1;
			}
		},

		/**
		 * @param {0|1} shaderStage 
		 * @param {number} sourcePtr 
		 * @param {number} sourceLen
		 * @returns {number}
		 */
		createShader: function (
			shaderStage,
			sourcePtr,
			sourceLen,
		) {
			let shader = gl.createShader(
				[gl.VERTEX_SHADER, gl.FRAGMENT_SHADER][shaderStage]
			);
			gl.shaderSource(
				shader,
				textDecoder.decode(context.metra.exports.memory.buffer.slice(sourcePtr, sourcePtr + sourceLen)),
			);
			gl.compileShader(shader);
			if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
				console.error(`Failed to compile ${["vertex", "fragment"][shaderStage]} shader!`, gl.getShaderInfoLog(shader));
				gl.deleteShader(shader);
				return 0;
			}

			let key = findKey();

			console.debug(`created shader with ID ${key}`);

			context.objects.set(key, shader);
			return key;
		},

		/**
		 * @param {number} shader
		 * @returns {0|1}
		 */
		dropShader: function (shader) {
			let obj = context.objects.get(shader);
			if (obj === undefined) {
				return 0;
			} else {
				gl.deleteBuffer(context.objects[shader]);
				context.objects.delete(shader);
				console.debug(`dropped shader with ID ${shader}`);
				return 1;
			}
		},

		/**
		 * @param {number} vertex
		 * @param {number} fragment 
		 * @returns {number}
		 */
		createProgram: function (
			vertex,
			fragment,
		) {
			let program = gl.createProgram();
			gl.attachShader(program, context.objects.get(vertex));
			gl.attachShader(program, context.objects.get(fragment));
			gl.linkProgram(program);

			if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
				console.error("Failed to link shader program!", gl.getProgramInfoLog(program));
				gl.deleteProgram(program);
				return 0;
			}

			let key = findKey();

			console.debug(`created program with ID ${key}`);

			context.objects.set(key, program);

			return key;
		},

		/**
		 * @param {number} program
		 * @returns {0|1}
		 */
		dropProgram: function (program) {
			let obj = context.objects.get(program);
			if (obj === undefined) {
				return 0;
			} else {
				gl.deleteBuffer(context.objects[program]);
				context.objects.delete(program);
				console.debug(`dropped program with ID ${program}`);
				return 1;
			}
		},

		/**
		 * @param {0|1|2} bufferType 
		 * @param {number} dataPtr 
		 * @param {number} dataLen 
		 * @returns {number}
		 */
		createBuffer: function (
			bufferType,
			dataPtr,
			dataLen,
		) {
			let buffer = gl.createBuffer();
			// This is enum conversion.
			let target = [gl.ARRAY_BUFFER, gl.ELEMENT_ARRAY_BUFFER, gl.UNIFORM_BUFFER][bufferType];
			// I've designed the API in such a way that makes it impossible to really re-use buffers
			// as the game developer, so pretty much every buffer will be STATIC_DRAW.
			let usage = gl.STATIC_DRAW;
			gl.bindBuffer(
				target,
				buffer,
			);
			gl.bufferData(
				target,
				context.metra.exports.memory.buffer.slice(dataPtr, dataPtr + dataLen),
				usage
			);
			
			let key = findKey();

			console.debug(`created buffer with ID ${key}`);

			context.objects.set(key, buffer);
			return key;
		},

		/**
		 * @param {number} buffer
		 * @returns {0|1}
		 */
		dropBuffer: function (buffer) {
			let obj = context.objects.get(buffer);
			if (obj === undefined) {
				return 0;
			} else {
				gl.deleteBuffer(context.objects[buffer]);
				// context.freeBuffers.push()
				context.objects.delete(buffer);
				console.debug(`dropped buffer with ID ${buffer}`);
				return 1;
			}
		}
	}
};

{
	console.info("Loading Metra...");
	console.group("Loading steps");
	let bt = performance.now();
	console.info(`Loading Metra WASM`);
	let metraSourcePromise = WebAssembly.instantiateStreaming(
		fetch("metra-game.wasm"),
		importObject
	);
	metraSourcePromise.then(() => {
		let delta = performance.now() - bt;
		console.info(`Loaded Metra WASM in ${delta}ms`);
	});

	let resourcePromise = Promise.all(Object.entries(resourceManifest).map((async value => {
		let resourceId = value[0];
		let resourceEntry = value[1];
		let canonicalPath = "./" + resourceEntry.path;
		switch (resourceEntry.resourceType) {
			case "sound": {
				console.info(`Loading sound file from "${canonicalPath}"`);
				let audioElement = new Audio(canonicalPath);
				audioElement.preload = "auto";
				return await new Promise((resolve, reject) => {
					audioElement.addEventListener('error', err => {
						let delta = performance.now() - bt;
						console.error(`Failed to load sound "${resourceId}" after ${delta}ms!`);
						reject(err);
					});
					audioElement.addEventListener('canplaythrough', () => {
						let delta = performance.now() - bt;
						console.info(`Loaded sound "${resourceId}" in ${delta}ms`);
						resolve(audioElement);
					});
				});
			}
			case "texture": {
				console.info(`Loading texture from "${canonicalPath}"`);
				let imgElement = new Image();
				imgElement.src = canonicalPath;
				return await new Promise((resolve, reject) => {
					// let hasErrored = false;
					// let hasLoaded = false;
					imgElement.addEventListener('error', err => {
						let delta = performance.now() - bt;
						console.error(`Failed to load texture \"${resourceId}\" after ${delta}ms!`);
						reject(err);
					});
					imgElement.addEventListener('load', () => {
						let delta = performance.now() - bt;
						console.info(`Loaded texture \"${resourceId}\" in ${delta}ms`);
						resolve(imgElement);
					});
				});
			}
			default: {
				console.error("Invalid ");
				return null;
			}
		}
	})));

	// resourceBank.
	// noinspection JSValidateTypes
	context.metra = (await metraSourcePromise).instance;
	await resourcePromise;
	console.groupEnd();

	let totalDelta = performance.now() - bt;
	console.info(`Done loading! Took ${totalDelta}ms total.`);
}

let eventQueue = [];

{
	/** @type {Gamepad[]} */
	let gamepads = [];
	/** @type {Map<string, boolean>} */
	let keyState = new Map();

	canvas.addEventListener('keydown', e => {
		// e.key
	});
	canvas.addEventListener('keyup', e => {

	});

	// TODO: future, gamepad support

	window.addEventListener("gamepadconnected", e => {
		gamepads.push(e.gamepad);
		console.log(`Gamepad connected at index ${e.gamepad.index}: ${e.gamepad.id}. ${e.gamepad.buttons.length} buttons, ${e.gamepad.axes.length} axes.`);
		console.log(gamepads);
	});

	window.addEventListener("gamepaddisconnected", e => {
		console.log(`Gamepad disconnected at index ${e.gamepad.index}: ${e.gamepad.id}.`);
		gamepads = gamepads.filter(v => v.index === e.gamepad.index);
		console.log(gamepads);
	});
}

let isRunning = true;

// initialize stuff
context.metra.exports.metraMain();

function update() {
	// gl.viewport
	let res = context.metra.exports.metraUpdate();
	if (res === 0) {
		isRunning = false;
	}

	gl.viewport(0, 0, canvas.width, canvas.height);
	gl.clearColor(0, 0, 0, 0);
	gl.clear(gl.COLOR_BUFFER_BIT /* | gl.DEPTH_BUFFER_BIT */);
	// gl.enable(gl.DEPTH_TEST);
	// gl.enable(gl.CULL_FACE);

	gl.flush();

	let error = gl.getError();
	// noinspection EqualityComparisonWithCoercionJS
	if (error !== gl.NO_ERROR) {
		console.error(`encountered WebGL error (code ${error})`);
	}

	if (isRunning) {
		requestAnimationFrame(update);
	} else {
		context.metra.exports.metraClean();
	}
}

function resize() {
	let bounds = canvas.getBoundingClientRect();
	canvas.width = bounds.width * window.devicePixelRatio;
	canvas.height = bounds.height * window.devicePixelRatio;
}
window.addEventListener('resize', resize);
resize();

update();