/**
 * SNEK Audio System
 * Professional synthwave music generation and sound effects
 */

import { CONFIG } from '../utils/config.js';

export class AudioSystem {
  constructor() {
    this.context = null;
    this.isMuted = false;
    this.currentMusic = null;
    this.musicTimeout = null;
    this.currentBPM = CONFIG.AUDIO.BASE_BPM;
    this.synthNodes = {};
    this.currentTrack = null;
    this.masterGain = null;
    this.compressor = null;
    this.reverb = null;
    
    this.init();
  }
  
  init() {
    try {
      // Use global AudioContext if available (for testing)
      const AudioContextClass = global.AudioContext || window.AudioContext || window.webkitAudioContext;
      this.context = new AudioContextClass();
      this.setupSynthNodes();
    } catch (error) {
      console.warn('Web Audio API not supported:', error);
      this.isMuted = true;
      this.context = null;
    }
  }
  
  setupSynthNodes() {
    if (!this.context) return;
    
    // Master audio chain
    this.masterGain = this.context.createGain();
    this.compressor = this.context.createDynamicsCompressor();
    this.reverb = this.context.createConvolver();
    
    // Master volume
    this.masterGain.gain.setValueAtTime(CONFIG.AUDIO.MASTER_VOLUME, this.context.currentTime);
    
    // Compressor settings for professional sound
    this.compressor.threshold.setValueAtTime(-24, this.context.currentTime);
    this.compressor.knee.setValueAtTime(30, this.context.currentTime);
    this.compressor.ratio.setValueAtTime(12, this.context.currentTime);
    this.compressor.attack.setValueAtTime(0.003, this.context.currentTime);
    this.compressor.release.setValueAtTime(0.25, this.context.currentTime);
    
    // Create reverb impulse response
    this.createReverbImpulse();
    
    // Individual channel gains for mixing
    this.synthNodes = {
      bass: this.context.createGain(),
      lead: this.context.createGain(),
      pad: this.context.createGain(),
      drums: this.context.createGain(),
      arp: this.context.createGain()
    };
    
    // Set channel volumes
    this.synthNodes.bass.gain.setValueAtTime(0.8, this.context.currentTime);
    this.synthNodes.lead.gain.setValueAtTime(0.6, this.context.currentTime);
    this.synthNodes.pad.gain.setValueAtTime(0.4, this.context.currentTime);
    this.synthNodes.drums.gain.setValueAtTime(0.7, this.context.currentTime);
    this.synthNodes.arp.gain.setValueAtTime(0.5, this.context.currentTime);
    
    // Connect audio chain: channels -> reverb -> compressor -> master -> destination
    Object.values(this.synthNodes).forEach(node => {
      node.connect(this.reverb);
      node.connect(this.compressor); // Dry signal
    });
    
    this.reverb.connect(this.compressor); // Wet signal
    this.compressor.connect(this.masterGain);
    this.masterGain.connect(this.context.destination);
  }
  
  createReverbImpulse() {
    if (!this.context) return;
    
    const sampleRate = this.context.sampleRate;
    const length = sampleRate * 2; // 2 second reverb
    const impulse = this.context.createBuffer(2, length, sampleRate);
    
    for (let channel = 0; channel < 2; channel++) {
      const channelData = impulse.getChannelData(channel);
      for (let i = 0; i < length; i++) {
        const decay = Math.pow(1 - i / length, 2);
        channelData[i] = (Math.random() * 2 - 1) * decay * 0.1;
      }
    }
    
    this.reverb.buffer = impulse;
  }
  
  createSynthVoice(frequency, startTime, duration, type = 'square', channel = 'lead', options = {}) {
    if (this.isMuted || !this.context) return null;
    
    const {
      volume = 0.5,
      attack = 0.01,
      decay = 0.1,
      sustain = 0.7,
      release = 0.2,
      filterFreq = 1000,
      filterQ = 1
    } = options;
    
    const osc = this.context.createOscillator();
    const gain = this.context.createGain();
    const filter = this.context.createBiquadFilter();
    
    // Oscillator setup
    osc.type = type;
    osc.frequency.setValueAtTime(frequency, startTime);
    
    // Filter setup (analog-style lowpass)
    filter.type = 'lowpass';
    filter.frequency.setValueAtTime(filterFreq, startTime);
    filter.Q.setValueAtTime(filterQ, startTime);
    
    // ADSR envelope
    gain.gain.setValueAtTime(0, startTime);
    gain.gain.linearRampToValueAtTime(volume, startTime + attack);
    gain.gain.linearRampToValueAtTime(volume * sustain, startTime + attack + decay);
    gain.gain.setValueAtTime(volume * sustain, startTime + duration - release);
    gain.gain.linearRampToValueAtTime(0, startTime + duration);
    
    // Connect audio chain
    osc.connect(filter);
    filter.connect(gain);
    gain.connect(this.synthNodes[channel] || this.synthNodes.lead);
    
    // Schedule playback
    osc.start(startTime);
    osc.stop(startTime + duration);
    
    return { osc, gain, filter };
  }
  
  playEatSound() {
    if (this.isMuted || !this.context) return;
    
    const now = this.context.currentTime;
    
    // Two-tone eating sound
    this.createSynthVoice(CONFIG.AUDIO.EAT_FREQ_1, now, CONFIG.AUDIO.EAT_DURATION, 'square', 'drums', {
      volume: CONFIG.AUDIO.SFX_VOLUME,
      attack: 0.01,
      release: 0.05
    });
    
    this.createSynthVoice(CONFIG.AUDIO.EAT_FREQ_2, now + 0.05, CONFIG.AUDIO.EAT_DURATION, 'square', 'drums', {
      volume: CONFIG.AUDIO.SFX_VOLUME * 0.7,
      attack: 0.01,
      release: 0.05
    });
  }
  
  playGameOverSound() {
    if (this.isMuted || !this.context) return;
    
    const now = this.context.currentTime;
    
    // Descending game over sequence
    this.createSynthVoice(CONFIG.AUDIO.GAME_OVER_FREQ_1, now, CONFIG.AUDIO.GAME_OVER_DURATION, 'square', 'drums', {
      volume: CONFIG.AUDIO.SFX_VOLUME,
      attack: 0.01,
      release: 0.1
    });
    
    this.createSynthVoice(CONFIG.AUDIO.GAME_OVER_FREQ_2, now + 0.2, CONFIG.AUDIO.GAME_OVER_DURATION, 'square', 'drums', {
      volume: CONFIG.AUDIO.SFX_VOLUME,
      attack: 0.01,
      release: 0.1
    });
    
    this.createSynthVoice(CONFIG.AUDIO.GAME_OVER_FREQ_3, now + 0.4, CONFIG.AUDIO.GAME_OVER_DURATION * 2, 'square', 'drums', {
      volume: CONFIG.AUDIO.SFX_VOLUME,
      attack: 0.01,
      release: 0.3
    });
  }
  
  startMenuMusic() {
    if (this.isMuted || !this.context) return;
    
    this.stopMusic();
    this.currentTrack = 'menu';
    this.currentBPM = CONFIG.AUDIO.BASE_BPM * 0.7; // Slower for menu
    this.playMenuTrack();
  }
  
  startGameplayMusic() {
    if (this.isMuted || !this.context) return;
    
    this.stopMusic();
    this.currentTrack = 'gameplay';
    this.currentBPM = CONFIG.AUDIO.BASE_BPM;
    this.playGameplayTrack();
  }
  
  playMenuTrack() {
    if (this.currentTrack !== 'menu' || this.isMuted) return;
    
    const now = this.context.currentTime;
    const beatDuration = 60 / this.currentBPM;
    
    // Ambient pad chords for menu
    this.createSynthVoice(220, now, beatDuration * 4, 'sine', 'pad', {
      volume: CONFIG.AUDIO.MUSIC_VOLUME * 0.3,
      attack: 0.5,
      decay: 0.5,
      sustain: 0.8,
      release: 1.0,
      filterFreq: 800
    });
    
    this.createSynthVoice(277, now, beatDuration * 4, 'sine', 'pad', {
      volume: CONFIG.AUDIO.MUSIC_VOLUME * 0.2,
      attack: 0.7,
      decay: 0.5,
      sustain: 0.7,
      release: 1.2,
      filterFreq: 600
    });
    
    // Schedule next iteration
    this.musicTimeout = setTimeout(() => this.playMenuTrack(), beatDuration * 4 * 1000);
  }
  
  playGameplayTrack() {
    if (this.currentTrack !== 'gameplay' || this.isMuted) return;
    
    const now = this.context.currentTime;
    const beatDuration = 60 / this.currentBPM;
    
    // Bass line
    this.createSynthVoice(110, now, beatDuration * 0.8, 'square', 'bass', {
      volume: CONFIG.AUDIO.MUSIC_VOLUME * 0.6,
      attack: 0.01,
      decay: 0.1,
      sustain: 0.3,
      release: 0.1,
      filterFreq: 200
    });
    
    // Lead synth melody
    const melodyFreqs = [440, 523, 659, 784];
    const melodyIndex = Math.floor(Math.random() * melodyFreqs.length);
    
    this.createSynthVoice(melodyFreqs[melodyIndex], now + beatDuration, beatDuration * 0.5, 'sawtooth', 'lead', {
      volume: CONFIG.AUDIO.MUSIC_VOLUME * 0.4,
      attack: 0.01,
      decay: 0.2,
      sustain: 0.6,
      release: 0.1,
      filterFreq: 2000,
      filterQ: 2
    });
    
    // Arpeggio
    if (Math.random() > 0.7) {
      [880, 1047, 1319].forEach((freq, i) => {
        this.createSynthVoice(freq, now + beatDuration * 2 + i * beatDuration * 0.25, beatDuration * 0.2, 'triangle', 'arp', {
          volume: CONFIG.AUDIO.MUSIC_VOLUME * 0.3,
          attack: 0.01,
          release: 0.05,
          filterFreq: 3000
        });
      });
    }
    
    // Schedule next iteration
    this.musicTimeout = setTimeout(() => this.playGameplayTrack(), beatDuration * 4 * 1000);
  }
  
  updateBPM(newBPM) {
    this.currentBPM = Math.min(newBPM, CONFIG.AUDIO.MAX_BPM);
  }
  
  stopMusic() {
    this.currentTrack = null;
    if (this.musicTimeout) {
      clearTimeout(this.musicTimeout);
      this.musicTimeout = null;
    }
  }
  
  toggleMute() {
    this.isMuted = !this.isMuted;
    if (this.isMuted) {
      this.stopMusic();
    }
    return this.isMuted;
  }
  
  setVolume(volume) {
    if (!this.masterGain) return;
    this.masterGain.gain.setValueAtTime(volume, this.context.currentTime);
  }
  
  destroy() {
    this.stopMusic();
    if (this.context && this.context.state !== 'closed' && typeof this.context.close === 'function') {
      this.context.close();
    }
  }
}

export default AudioSystem;