/**
 * Audio System Tests
 * Tests for the synthwave audio engine
 */

import AudioSystem from '../src/systems/AudioSystem.js';
import { CONFIG } from '../src/utils/config.js';

describe('Audio System', () => {
  let audioSystem;
  
  beforeEach(() => {
    // Suppress console warnings in tests
    jest.spyOn(console, 'warn').mockImplementation(() => {});
    audioSystem = new AudioSystem();
  });
  
  afterEach(() => {
    if (audioSystem) {
      audioSystem.destroy();
    }
    jest.restoreAllMocks();
  });
  
  test('should initialize audio system correctly', () => {
    expect(audioSystem.context).toBeDefined();
    expect(typeof audioSystem.isMuted).toBe('boolean');
    expect(audioSystem.currentBPM).toBe(CONFIG.AUDIO.BASE_BPM);
    expect(audioSystem.currentTrack).toBe(null);
  });
  
  test('should setup synth nodes correctly', () => {
    if (audioSystem.context && !audioSystem.isMuted) {
      expect(audioSystem.masterGain).toBeDefined();
      expect(audioSystem.compressor).toBeDefined();
      expect(audioSystem.reverb).toBeDefined();
      
      const expectedChannels = ['bass', 'lead', 'pad', 'drums', 'arp'];
      expectedChannels.forEach(channel => {
        expect(audioSystem.synthNodes[channel]).toBeDefined();
      });
    } else {
      // When Web Audio API is not available or muted, check graceful degradation
      expect(audioSystem.currentTrack).toBe(null);
    }
  });
  
  test('should create synth voices with correct parameters', () => {
    const voice = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead', {
      volume: 0.5,
      attack: 0.1,
      decay: 0.2,
      sustain: 0.7,
      release: 0.3
    });
    
    if (!audioSystem.isMuted) {
      expect(voice).not.toBe(null);
      expect(voice.osc).toBeDefined();
      expect(voice.gain).toBeDefined();
      expect(voice.filter).toBeDefined();
    }
  });
  
  test('should handle mute functionality', () => {
    audioSystem.isMuted = false;
    const result = audioSystem.toggleMute();
    expect(result).toBe(true);
    expect(audioSystem.isMuted).toBe(true);
    
    audioSystem.toggleMute();
    expect(audioSystem.isMuted).toBe(false);
  });
  
  test('should play sound effects correctly', () => {
    audioSystem.isMuted = false;
    
    // Test that sound functions don't throw errors
    expect(() => audioSystem.playEatSound()).not.toThrow();
    expect(() => audioSystem.playGameOverSound()).not.toThrow();
  });
  
  test('should manage music tracks correctly', () => {
    if (audioSystem.context && !audioSystem.isMuted) {
      audioSystem.startMenuMusic();
      expect(audioSystem.currentTrack).toBe('menu');
      
      audioSystem.startGameplayMusic();
      expect(audioSystem.currentTrack).toBe('gameplay');
    }
    
    audioSystem.stopMusic();
    expect(audioSystem.currentTrack).toBe(null);
  });
  
  test('should respect mute state for sound generation', () => {
    audioSystem.isMuted = true;
    const voice = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead');
    expect(voice).toBe(null);
    
    if (audioSystem.context) {
      audioSystem.isMuted = false;
      const voice2 = audioSystem.createSynthVoice(440, 0, 1.0, 'square', 'lead');
      expect(voice2).not.toBe(null);
    }
  });
  
  test('should update BPM correctly', () => {
    const newBPM = 150;
    audioSystem.updateBPM(newBPM);
    expect(audioSystem.currentBPM).toBe(newBPM);
    
    // Test max BPM limit
    audioSystem.updateBPM(CONFIG.AUDIO.MAX_BPM + 50);
    expect(audioSystem.currentBPM).toBe(CONFIG.AUDIO.MAX_BPM);
  });
  
  test('should handle destruction properly', () => {
    expect(() => audioSystem.destroy()).not.toThrow();
    expect(audioSystem.currentTrack).toBe(null);
  });
  
});