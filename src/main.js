import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { listen } from '@tauri-apps/api/event';

const dropZone = document.getElementById('dropZone');
const dropText = document.getElementById('dropText');
const fileNameEl = document.getElementById('fileName');
const createBtn = document.getElementById('createBtn');
const progress = document.getElementById('progress');
const progressFill = document.getElementById('progressFill');
const status = document.getElementById('status');
const languageSelect = document.getElementById('language');
const musicStartInput = document.getElementById('musicStart');
const musicEndInput = document.getElementById('musicEnd');
const silenceStartInput = document.getElementById('silenceStart');
const silenceEndInput = document.getElementById('silenceEnd');

let selectedFilePath = null;

console.log('App inizializzata');

// Ascolta eventi di progresso dal backend
listen('transcription-progress', (event) => {
  const { percent, message } = event.payload;
  progressFill.style.width = `${percent}%`;
  status.textContent = message;
});

// Click per selezionare file
dropZone.addEventListener('click', async () => {
  console.log('Click rilevato');
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Media',
        extensions: ['mp4', 'mp3', 'wav', 'm4a', 'webm', 'ogg']
      }]
    });
    
    console.log('File selezionato:', selected);
    
    if (selected) {
      selectedFilePath = selected;
      const fileName = selected.split('/').pop().split('\\').pop();
      dropText.textContent = 'File selezionato:';
      fileNameEl.textContent = fileName;
      createBtn.disabled = false;
      dropZone.classList.add('success');
    }
  } catch (err) {
    console.error('Errore apertura dialog:', err);
  }
});

// Click sul pulsante Crea VTT
createBtn.addEventListener('click', async () => {
  if (selectedFilePath) {
    await processFile(selectedFilePath);
  }
});

async function processFile(filePath) {
  const language = languageSelect.value;
  const musicStart = parseFloat(musicStartInput.value) || 0;
  const musicEnd = parseFloat(musicEndInput.value) || 0;
  const silenceStart = parseFloat(silenceStartInput.value) || 0;
  const silenceEnd = parseFloat(silenceEndInput.value) || 0;
  
  console.log('Processo file:', filePath, 'lingua:', language, 'musica:', musicStart, '-', musicEnd, 'silenzio:', silenceStart, '-', silenceEnd);
  
  dropZone.classList.add('processing');
  dropZone.classList.remove('success', 'error');
  progress.classList.add('visible');
  progressFill.style.width = '0%';
  status.textContent = 'Avvio elaborazione...';
  
  try {
    const result = await invoke('transcribe', { 
      filePath,
      language,
      musicStart,
      musicEnd,
      silenceStart,
      silenceEnd
    });
    
    console.log('Risultato:', result);
    
    dropZone.classList.remove('processing');
    dropZone.classList.add('success');
    progressFill.style.width = '100%';
    status.textContent = `✓ Completato: ${result}`;
    
    setTimeout(() => {
      dropZone.classList.remove('success');
      progress.classList.remove('visible');
      // Reset per nuova selezione
      selectedFilePath = null;
      dropText.textContent = 'Fai click per selezionare un file';
      fileNameEl.textContent = 'MP4, MP3, WAV, M4A';
      createBtn.disabled = true;
    }, 5000);
    
  } catch (error) {
    console.error('Errore trascrizione:', error);
    dropZone.classList.remove('processing');
    dropZone.classList.add('error');
    status.textContent = `✗ Errore: ${error}`;
    
    setTimeout(() => {
      dropZone.classList.remove('error');
    }, 5000);
  }
}
