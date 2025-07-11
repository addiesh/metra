const audioCtx = new AudioContext();

/** @type {[]} */
let activeBurstSounds = [];

/** @type {[]} */
let activeDroneSounds = [];

audioCtx.createMediaElementSource();