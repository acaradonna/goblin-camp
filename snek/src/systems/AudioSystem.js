/**
 * SNEK Audio System
 * Dark synthwave music generation with authentic 80s cyberpunk sound
 * Inspired by Robocop, Blade Runner, and classic darkwave
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
    this.delayNode = null;
    
    // Music state
    this.sequencePosition = 0;
    this.patternLength = 16;
    this.currentKey = 'Am'; // A minor for dark synthwave
    this.intensity = 0; // 0-1, affects music complexity and BPM
    
    // Dark synthwave scales and progressions
    this.scales = {
      // A harmonic minor for that dark cyberpunk feel
      'Am_harmonic': [220, 246.94, 261.63, 293.66, 329.63, 349.23, 415.30, 440],
      // A phrygian for exotic darkness
      'A_phrygian': [220, 233.08, 261.63, 293.66, 329.63, 349.23, 392, 440],
      // Natural minor for classic darkwave
      'Am_natural': [220, 246.94, 261.63, 293.66, 329.63, 349.23, 392, 440]
    };
    
    // Dark chord progressions (i-VI-III-VII, i-VII-VI-V, etc.)
    this.chordProgressions = {
      'dark_cyberpunk': [
        [220, 261.63, 329.63], // Am
        [349.23, 415.30, 523.25], // F
        [261.63, 329.63, 392], // C  
        [392, 466.16, 587.33] // G
      ],
      'robocop_style': [
        [220, 261.63, 329.63], // Am
        [392, 466.16, 587.33], // G
        [349.23, 415.30, 523.25], // F
        [329.63, 392, 493.88] // E
      ],
      'phrygian_darkness': [
        [220, 261.63, 329.63], // Am
        [233.08, 293.66, 349.23], // Bb
        [261.63, 329.63, 392], // C
        [220, 261.63, 329.63] // Am
      ]
    };
    
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
    
    // Master audio chain with 80s-style effects
    this.masterGain = this.context.createGain();
    this.compressor = this.context.createDynamicsCompressor();
    this.reverb = this.context.createConvolver();
    this.delayNode = this.context.createDelay();
    this.delayFeedback = this.context.createGain();
    this.delayWet = this.context.createGain();
    
    // Master volume
    this.masterGain.gain.setValueAtTime(CONFIG.AUDIO.MASTER_VOLUME, this.context.currentTime);
    
    // 80s-style compressor settings for that punchy sound
    this.compressor.threshold.setValueAtTime(-18, this.context.currentTime);
    this.compressor.knee.setValueAtTime(20, this.context.currentTime);
    this.compressor.ratio.setValueAtTime(8, this.context.currentTime);
    this.compressor.attack.setValueAtTime(0.001, this.context.currentTime);
    this.compressor.release.setValueAtTime(0.1, this.context.currentTime);
    
    // Analog-style delay (essential for synthwave)
    this.delayNode.delayTime.setValueAtTime(0.25, this.context.currentTime); // Quarter note delay
    this.delayFeedback.gain.setValueAtTime(0.3, this.context.currentTime);
    this.delayWet.gain.setValueAtTime(0.2, this.context.currentTime);
    
    // Create dark, spacious reverb
    this.createDarkReverbImpulse();
    
    // Individual channel gains for professional mixing
    this.synthNodes = {
      bass: this.context.createGain(),
      lead: this.context.createGain(),
      pad: this.context.createGain(),
      drums: this.context.createGain(),
      arp: this.context.createGain(),
      sfx: this.context.createGain()
    };
    
    // Dark synthwave channel mix levels
    this.synthNodes.bass.gain.setValueAtTime(0.9, this.context.currentTime);
    this.synthNodes.lead.gain.setValueAtTime(0.7, this.context.currentTime);
    this.synthNodes.pad.gain.setValueAtTime(0.5, this.context.currentTime);
    this.synthNodes.drums.gain.setValueAtTime(0.8, this.context.currentTime);
    this.synthNodes.arp.gain.setValueAtTime(0.4, this.context.currentTime);
    this.synthNodes.sfx.gain.setValueAtTime(0.6, this.context.currentTime);
    
    // Setup delay chain
    this.delayNode.connect(this.delayFeedback);
    this.delayFeedback.connect(this.delayNode);
    this.delayNode.connect(this.delayWet);
    
    // Connect audio chain: channels -> delay -> reverb -> compressor -> master -> destination
    Object.values(this.synthNodes).forEach(node => {
      node.connect(this.delayNode); // Send to delay
      node.connect(this.reverb); // Send to reverb
      node.connect(this.compressor); // Dry signal
    });
    
    this.delayWet.connect(this.reverb);
    this.reverb.connect(this.compressor);
    this.compressor.connect(this.masterGain);
    this.masterGain.connect(this.context.destination);
  }
  
  createDarkReverbImpulse() {
    if (!this.context) return;
    
    const sampleRate = this.context.sampleRate;
    const length = sampleRate * 3; // 3 second dark, spacious reverb
    const impulse = this.context.createBuffer(2, length, sampleRate);
    
    for (let channel = 0; channel < 2; channel++) {
      const channelData = impulse.getChannelData(channel);
      for (let i = 0; i < length; i++) {
        // Create dark, metallic reverb with multiple decay curves
        const n = length - i;
        const t = i / length;
        
        // Primary exponential decay
        const decay1 = Math.pow(1 - t, 1.5);
        
        // Secondary logarithmic decay for metallic character
        const decay2 = Math.log(1 + n / length * 9) / Math.log(10);
        
        // Add some early reflections for that concrete/metal space feel
        const earlyReflections = Math.sin(t * Math.PI * 20) * Math.exp(-t * 10) * 0.3;
        
        // Combine for dark, cyberpunk atmosphere
        const sample = (Math.random() * 2 - 1) * (decay1 * 0.7 + decay2 * 0.3) * 0.15 + earlyReflections;
        
        // Pan slightly for stereo width
        channelData[i] = sample * (channel === 0 ? 1 : 0.8);
      }
    }
    
    this.reverb.buffer = impulse;
  }
  
  createAnalogSynth(frequency, startTime, duration, preset = 'dark_lead', channel = 'lead', options = {}) {
    if (this.isMuted || !this.context) return null;
    
    // Analog synth presets for authentic 80s sound
    const presets = {
      'dark_lead': {
        oscillators: [
          { type: 'sawtooth', detune: -3, volume: 0.7 },
          { type: 'sawtooth', detune: 3, volume: 0.7 }
        ],
        filter: { type: 'lowpass', frequency: 1200, Q: 8, envelope: 0.3 },
        envelope: { attack: 0.01, decay: 0.2, sustain: 0.6, release: 0.4 },
        lfo: { rate: 4, depth: 20, target: 'filter' }
      },
      'bass_synth': {
        oscillators: [
          { type: 'sawtooth', detune: 0, volume: 0.9 },
          { type: 'square', detune: -12, volume: 0.3 }
        ],
        filter: { type: 'lowpass', frequency: 400, Q: 5, envelope: 0.2 },
        envelope: { attack: 0.01, decay: 0.1, sustain: 0.3, release: 0.15 },
        lfo: { rate: 0.5, depth: 10, target: 'pitch' }
      },
      'dark_pad': {
        oscillators: [
          { type: 'sawtooth', detune: -7, volume: 0.4 },
          { type: 'sawtooth', detune: 0, volume: 0.4 },
          { type: 'sawtooth', detune: 7, volume: 0.4 }
        ],
        filter: { type: 'lowpass', frequency: 800, Q: 2, envelope: 0.4 },
        envelope: { attack: 0.8, decay: 0.5, sustain: 0.8, release: 1.2 },
        lfo: { rate: 0.3, depth: 40, target: 'filter' }
      },
      'arp_pluck': {
        oscillators: [
          { type: 'sawtooth', detune: 0, volume: 0.8 }
        ],
        filter: { type: 'lowpass', frequency: 2500, Q: 12, envelope: 0.7 },
        envelope: { attack: 0.005, decay: 0.1, sustain: 0.2, release: 0.1 },
        lfo: { rate: 8, depth: 100, target: 'filter' }
      }
    };
    
    const config = { ...presets[preset], ...options };
    const voices = [];
    
    // Create multiple oscillators for analog thickness
    config.oscillators.forEach(oscConfig => {
      const osc = this.context.createOscillator();
      const gain = this.context.createGain();
      const filter = this.context.createBiquadFilter();
      
      // Oscillator setup with detuning for analog character
      osc.type = oscConfig.type;
      const detuneFreq = frequency * Math.pow(2, oscConfig.detune / 1200);
      osc.frequency.setValueAtTime(detuneFreq, startTime);
      
      // Analog-style filter with resonance
      filter.type = config.filter.type;
      const filterFreq = config.filter.frequency;
      filter.frequency.setValueAtTime(filterFreq, startTime);
      filter.Q.setValueAtTime(config.filter.Q, startTime);
      
      // Filter envelope for movement
      const filterEnvAmount = config.filter.envelope * filterFreq;
      filter.frequency.linearRampToValueAtTime(
        filterFreq + filterEnvAmount, 
        startTime + config.envelope.attack + config.envelope.decay
      );
      filter.frequency.exponentialRampToValueAtTime(
        filterFreq, 
        startTime + duration - config.envelope.release
      );
      
      // LFO for analog movement
      if (config.lfo) {
        const lfo = this.context.createOscillator();
        const lfoGain = this.context.createGain();
        
        lfo.type = 'sine';
        lfo.frequency.setValueAtTime(config.lfo.rate, startTime);
        lfoGain.gain.setValueAtTime(config.lfo.depth, startTime);
        
        lfo.connect(lfoGain);
        if (config.lfo.target === 'filter') {
          lfoGain.connect(filter.frequency);
        } else if (config.lfo.target === 'pitch') {
          lfoGain.connect(osc.frequency);
        }
        
        lfo.start(startTime);
        lfo.stop(startTime + duration);
      }
      
      // ADSR envelope with exponential curves for analog feel
      gain.gain.setValueAtTime(0, startTime);
      gain.gain.exponentialRampToValueAtTime(
        oscConfig.volume * 0.01, startTime + 0.001
      );
      gain.gain.exponentialRampToValueAtTime(
        oscConfig.volume, startTime + config.envelope.attack
      );
      gain.gain.exponentialRampToValueAtTime(
        oscConfig.volume * config.envelope.sustain, 
        startTime + config.envelope.attack + config.envelope.decay
      );
      gain.gain.setValueAtTime(
        oscConfig.volume * config.envelope.sustain, 
        startTime + duration - config.envelope.release
      );
      gain.gain.exponentialRampToValueAtTime(
        0.001, startTime + duration
      );
      
      // Connect audio chain
      osc.connect(filter);
      filter.connect(gain);
      gain.connect(this.synthNodes[channel] || this.synthNodes.lead);
      
      // Schedule playback
      osc.start(startTime);
      osc.stop(startTime + duration);
      
      voices.push({ osc, gain, filter });
    });
    
    return voices;
  }
  
  playCyberpunkEatSound() {
    if (this.isMuted || !this.context) return;
    
    const now = this.context.currentTime;
    
    // Digital glitch-style eating sound with pitch sweep
    this.createAnalogSynth(800, now, 0.15, 'arp_pluck', 'sfx', {
      oscillators: [
        { type: 'square', detune: 0, volume: 0.8 }
      ],
      filter: { type: 'lowpass', frequency: 3000, Q: 20, envelope: 0.9 },
      envelope: { attack: 0.001, decay: 0.1, sustain: 0.1, release: 0.05 }
    });
    
    // Add harmonic sweep for that digital feel
    setTimeout(() => {
      this.createAnalogSynth(1200, now + 0.05, 0.1, 'arp_pluck', 'sfx', {
        oscillators: [
          { type: 'square', detune: 0, volume: 0.6 }
        ],
        filter: { type: 'highpass', frequency: 800, Q: 8, envelope: 0.5 },
        envelope: { attack: 0.001, decay: 0.05, sustain: 0.2, release: 0.05 }
      });
    }, 0);
  }
  
  playRobotDeathSound() {
    if (this.isMuted || !this.context) return;
    
    const now = this.context.currentTime;
    
    // Dramatic descending robotic death sequence
    const deathChord = [220, 174, 146]; // Dark minor chord descent
    
    deathChord.forEach((freq, i) => {
      this.createAnalogSynth(freq, now + i * 0.3, 0.8, 'bass_synth', 'sfx', {
        oscillators: [
          { type: 'sawtooth', detune: 0, volume: 0.9 },
          { type: 'square', detune: -12, volume: 0.4 }
        ],
        filter: { type: 'lowpass', frequency: 400 - i * 100, Q: 8, envelope: 0.3 },
        envelope: { attack: 0.05, decay: 0.2, sustain: 0.4, release: 0.6 },
        lfo: { rate: 1 + i, depth: 50, target: 'filter' }
      });
    });
    
    // Add digital glitch noise
    setTimeout(() => {
      for (let j = 0; j < 5; j++) {
        this.createAnalogSynth(Math.random() * 1000 + 200, now + 0.5 + j * 0.1, 0.05, 'arp_pluck', 'sfx', {
          oscillators: [
            { type: 'square', detune: Math.random() * 100 - 50, volume: 0.3 }
          ],
          filter: { type: 'bandpass', frequency: Math.random() * 2000 + 500, Q: 20, envelope: 0.8 },
          envelope: { attack: 0.001, decay: 0.02, sustain: 0.1, release: 0.03 }
        });
      }
    }, 0);
  }
  
  startMenuMusic() {
    if (this.isMuted || !this.context) return;
    
    this.stopMusic();
    this.currentTrack = 'menu';
    this.currentBPM = 70; // Slow, brooding menu tempo
    this.sequencePosition = 0;
    this.intensity = 0.1;
    this.playDarkAmbientTrack();
  }
  
  startGameplayMusic() {
    if (this.isMuted || !this.context) return;
    
    this.stopMusic();
    this.currentTrack = 'gameplay';
    this.currentBPM = 120; // Start at base tempo
    this.sequencePosition = 0;
    this.intensity = 0.3;
    this.playDarkSynthwaveTrack();
  }
  
  playDarkAmbientTrack() {
    if (this.currentTrack !== 'menu' || this.isMuted) return;
    
    const now = this.context.currentTime;
    const beatDuration = 60 / this.currentBPM;
    const chords = this.chordProgressions['phrygian_darkness'];
    const chordIndex = this.sequencePosition % chords.length;
    
    // Dark ambient pad - builds atmosphere like Blade Runner
    chords[chordIndex].forEach((freq, i) => {
      this.createAnalogSynth(freq * 0.5, now + i * 0.1, beatDuration * 3, 'dark_pad', 'pad', {
        oscillators: [
          { type: 'sawtooth', detune: -5, volume: 0.3 },
          { type: 'sawtooth', detune: 0, volume: 0.3 },
          { type: 'sawtooth', detune: 5, volume: 0.3 }
        ],
        filter: { type: 'lowpass', frequency: 600, Q: 1, envelope: 0.3 },
        envelope: { attack: 1.5, decay: 1.0, sustain: 0.9, release: 2.0 },
        lfo: { rate: 0.2, depth: 30, target: 'filter' }
      });
    });
    
    // Subtle bass pulse - like a heartbeat in the machine
    if (this.sequencePosition % 2 === 0) {
      this.createAnalogSynth(55, now, beatDuration * 0.6, 'bass_synth', 'bass', {
        oscillators: [
          { type: 'sine', detune: 0, volume: 0.8 }
        ],
        filter: { type: 'lowpass', frequency: 80, Q: 2, envelope: 0.1 },
        envelope: { attack: 0.1, decay: 0.3, sustain: 0.2, release: 0.4 }
      });
    }
    
    // Occasional dark melody fragments
    if (Math.random() > 0.7) {
      const scale = this.scales['A_phrygian'];
      const noteIndex = Math.floor(Math.random() * scale.length);
      this.createAnalogSynth(scale[noteIndex] * 2, now + beatDuration, beatDuration * 0.8, 'dark_lead', 'lead', {
        oscillators: [
          { type: 'sawtooth', detune: 0, volume: 0.4 }
        ],
        filter: { type: 'lowpass', frequency: 1500, Q: 4, envelope: 0.4 },
        envelope: { attack: 0.3, decay: 0.5, sustain: 0.6, release: 0.8 }
      });
    }
    
    this.sequencePosition++;
    this.musicTimeout = setTimeout(() => this.playDarkAmbientTrack(), beatDuration * 4 * 1000);
  }
  
  playDarkSynthwaveTrack() {
    if (this.currentTrack !== 'gameplay' || this.isMuted) return;
    
    const now = this.context.currentTime;
    const beatDuration = 60 / this.currentBPM;
    const chords = this.chordProgressions['robocop_style'];
    const chordIndex = Math.floor(this.sequencePosition / 4) % chords.length;
    const beatInPattern = this.sequencePosition % 16;
    
    // Driving bass line - The foundation of dark synthwave
    if (beatInPattern % 4 === 0 || beatInPattern % 4 === 2) {
      const bassNote = chords[chordIndex][0] * 0.5; // Root note, octave down
      this.createAnalogSynth(bassNote, now, beatDuration * 0.7, 'bass_synth', 'bass', {
        oscillators: [
          { type: 'sawtooth', detune: 0, volume: 0.9 },
          { type: 'square', detune: -12, volume: 0.4 }
        ],
        filter: { type: 'lowpass', frequency: 300 + this.intensity * 200, Q: 5, envelope: 0.2 },
        envelope: { attack: 0.01, decay: 0.1, sustain: 0.3, release: 0.15 },
        lfo: { rate: 0.5, depth: 20, target: 'filter' }
      });
    }
    
    // Dark chord stabs - Robocop-style brass-like synth hits
    if (beatInPattern % 8 === 0) {
      chords[chordIndex].forEach((freq, i) => {
        this.createAnalogSynth(freq, now + i * 0.02, beatDuration * 2, 'dark_pad', 'pad', {
          oscillators: [
            { type: 'sawtooth', detune: -3, volume: 0.4 },
            { type: 'sawtooth', detune: 3, volume: 0.4 }
          ],
          filter: { type: 'lowpass', frequency: 800 + this.intensity * 400, Q: 3, envelope: 0.3 },
          envelope: { attack: 0.05, decay: 0.3, sustain: 0.7, release: 1.0 },
          lfo: { rate: 1, depth: 40, target: 'filter' }
        });
      });
    }
    
    // Menacing lead melody - Uses harmonic minor for that evil sound
    if (beatInPattern % 8 === 4 && this.intensity > 0.4) {
      const scale = this.scales['Am_harmonic'];
      const melodyNotes = [scale[4], scale[6], scale[7], scale[5]]; // Creates tension
      const noteIndex = Math.floor(this.sequencePosition / 8) % melodyNotes.length;
      
      this.createAnalogSynth(melodyNotes[noteIndex] * 2, now, beatDuration * 1.5, 'dark_lead', 'lead', {
        oscillators: [
          { type: 'sawtooth', detune: -2, volume: 0.6 },
          { type: 'sawtooth', detune: 2, volume: 0.6 }
        ],
        filter: { type: 'lowpass', frequency: 1200 + this.intensity * 800, Q: 8, envelope: 0.4 },
        envelope: { attack: 0.02, decay: 0.2, sustain: 0.6, release: 0.4 },
        lfo: { rate: 6, depth: 50, target: 'filter' }
      });
    }
    
    // Arpeggiated sequences - Classic synthwave texture
    if (beatInPattern % 4 === 1 && Math.random() > 0.6) {
      const arpNotes = [chords[chordIndex][0] * 2, chords[chordIndex][1] * 2, chords[chordIndex][2] * 2];
      arpNotes.forEach((freq, i) => {
        this.createAnalogSynth(freq, now + i * beatDuration * 0.25, beatDuration * 0.2, 'arp_pluck', 'arp', {
          oscillators: [
            { type: 'sawtooth', detune: 0, volume: 0.5 }
          ],
          filter: { type: 'lowpass', frequency: 2500 + i * 500, Q: 12, envelope: 0.7 },
          envelope: { attack: 0.005, decay: 0.1, sustain: 0.2, release: 0.1 }
        });
      });
    }
    
    // 80s drum machine hits
    if (beatInPattern % 4 === 0) {
      this.create808Kick(now);
    }
    if (beatInPattern % 4 === 2) {
      this.createSynthSnare(now + beatDuration * 0.05);
    }
    if (beatInPattern % 2 === 1) {
      this.createHiHat(now + beatDuration * 0.1);
    }
    
    this.sequencePosition++;
    this.musicTimeout = setTimeout(() => this.playDarkSynthwaveTrack(), beatDuration * 1000);
  }
  
  create808Kick(startTime) {
    // Classic 808-style kick drum
    this.createAnalogSynth(60, startTime, 0.3, 'bass_synth', 'drums', {
      oscillators: [
        { type: 'sine', detune: 0, volume: 1.0 }
      ],
      filter: { type: 'lowpass', frequency: 100, Q: 2, envelope: 0.3 },
      envelope: { attack: 0.001, decay: 0.1, sustain: 0.1, release: 0.2 }
    });
    
    // Add punch with higher frequency component
    this.createAnalogSynth(120, startTime, 0.05, 'arp_pluck', 'drums', {
      oscillators: [
        { type: 'square', detune: 0, volume: 0.4 }
      ],
      filter: { type: 'lowpass', frequency: 200, Q: 10, envelope: 0.8 },
      envelope: { attack: 0.001, decay: 0.03, sustain: 0.1, release: 0.02 }
    });
  }
  
  createSynthSnare(startTime) {
    // Synthetic snare with noise and tone
    this.createAnalogSynth(200, startTime, 0.15, 'arp_pluck', 'drums', {
      oscillators: [
        { type: 'square', detune: 0, volume: 0.6 }
      ],
      filter: { type: 'bandpass', frequency: 2000, Q: 8, envelope: 0.5 },
      envelope: { attack: 0.001, decay: 0.08, sustain: 0.1, release: 0.07 }
    });
    
    // Add noise component
    for (let i = 0; i < 3; i++) {
      this.createAnalogSynth(Math.random() * 3000 + 1000, startTime + i * 0.01, 0.05, 'arp_pluck', 'drums', {
        oscillators: [
          { type: 'square', detune: Math.random() * 200 - 100, volume: 0.2 }
        ],
        filter: { type: 'highpass', frequency: 8000, Q: 5, envelope: 0.3 },
        envelope: { attack: 0.001, decay: 0.02, sustain: 0.1, release: 0.03 }
      });
    }
  }
  
  createHiHat(startTime) {
    // Crispy hi-hat
    for (let i = 0; i < 2; i++) {
      this.createAnalogSynth(Math.random() * 8000 + 8000, startTime, 0.08, 'arp_pluck', 'drums', {
        oscillators: [
          { type: 'square', detune: Math.random() * 100 - 50, volume: 0.15 }
        ],
        filter: { type: 'highpass', frequency: 12000, Q: 2, envelope: 0.2 },
        envelope: { attack: 0.001, decay: 0.04, sustain: 0.1, release: 0.04 }
      });
    }
  }
  
  updateBPM(newBPM) {
    this.currentBPM = Math.min(newBPM, CONFIG.AUDIO.MAX_BPM);
    
    // Update delay time to match new tempo
    if (this.delayNode && this.context) {
      const newDelayTime = 60 / this.currentBPM / 4; // Quarter note delay
      this.delayNode.delayTime.exponentialRampToValueAtTime(
        Math.max(newDelayTime, 0.01), 
        this.context.currentTime + 0.1
      );
    }
  }
  
  updateIntensity(score, snakeLength) {
    // Calculate intensity based on game progression (0-1)
    const scoreIntensity = Math.min(score / 1000, 0.7);
    const lengthIntensity = Math.min(snakeLength / 50, 0.5);
    this.intensity = Math.min(scoreIntensity + lengthIntensity, 1.0);
    
    // Increase BPM with intensity
    if (this.currentTrack === 'gameplay') {
      const newBPM = 120 + (this.intensity * 60); // 120-180 BPM range
      this.updateBPM(newBPM);
    }
    
    // Update delay feedback for more chaos at high intensity
    if (this.delayFeedback && this.context) {
      const feedbackAmount = 0.2 + (this.intensity * 0.3);
      this.delayFeedback.gain.exponentialRampToValueAtTime(
        feedbackAmount, 
        this.context.currentTime + 0.5
      );
    }
  }
  
  // Backward compatibility methods
  playEatSound() {
    this.playCyberpunkEatSound();
  }
  
  playGameOverSound() {
    this.playRobotDeathSound();
  }
  
  // Legacy method for compatibility
  createSynthVoice(frequency, startTime, duration, type = 'sawtooth', channel = 'lead', options = {}) {
    // Map old parameters to new analog synth system
    const preset = channel === 'bass' ? 'bass_synth' : 
                   channel === 'pad' ? 'dark_pad' : 
                   channel === 'arp' ? 'arp_pluck' : 'dark_lead';
    
    return this.createAnalogSynth(frequency, startTime, duration, preset, channel, {
      oscillators: [
        { type: type, detune: 0, volume: options.volume || 0.5 }
      ],
      filter: { 
        type: 'lowpass', 
        frequency: options.filterFreq || 1000, 
        Q: options.filterQ || 1, 
        envelope: 0.3 
      },
      envelope: { 
        attack: options.attack || 0.01, 
        decay: options.decay || 0.1, 
        sustain: options.sustain || 0.7, 
        release: options.release || 0.2 
      }
    });
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
    } else {
      // Resume music based on current context
      if (this.currentTrack === 'menu') {
        this.startMenuMusic();
      } else if (this.currentTrack === 'gameplay') {
        this.startGameplayMusic();
      }
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