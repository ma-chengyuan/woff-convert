import "./style.css";

import { convert_ttf_to_woff2, convert_woff2_to_ttf } from "woff-convert";

const brotliLevelInput = document.getElementById(
  "brotli-level",
) as HTMLInputElement;
const ttfToWoffButton = document.getElementById(
  "ttf-to-woff",
) as HTMLButtonElement;
const woffToTtfButton = document.getElementById(
  "woff-to-ttf",
) as HTMLButtonElement;
const autoDownloadCheckbox = document.getElementById(
  "auto-download",
) as HTMLInputElement;
const resultsDiv = document.getElementById("convert-results") as HTMLDivElement;

function upload(callback: (file: Uint8Array, filename: string) => void) {
  // Create a file input element
  const fileInput = document.createElement("input");
  fileInput.type = "file";
  fileInput.accept = "*/*";
  fileInput.multiple = false;
  fileInput.style.display = "none";
  document.body.appendChild(fileInput);
  fileInput.click();
  // Upload and get file as Uint8Array
  fileInput.addEventListener("change", async () => {
    const file = fileInput.files?.[0];
    if (file) {
      const arrayBuffer = await file.arrayBuffer();
      const data = new Uint8Array(arrayBuffer);
      callback(data, file.name);
    }
    document.body.removeChild(fileInput);
  });
}

function download(data: Uint8Array, filename: string) {
  const blob = new Blob([data], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

function waitUntilDOMUpdated(callback: () => void) {
  requestAnimationFrame(() => {
    requestAnimationFrame(callback);
  });
}

ttfToWoffButton.addEventListener("click", () => {
  upload((ttf, filename) => {
    const level = parseInt(brotliLevelInput.value);
    resultsDiv.innerHTML = `<p>Converting TTF with ${ttf.length} bytes to WOFF2 (Brotli level ${level})...</p>`;
    waitUntilDOMUpdated(() => {
      try {
        const start = performance.now();
        const woff2 = convert_ttf_to_woff2(ttf, level);
        const elapsed = performance.now() - start;
        resultsDiv.innerHTML += `<p>Converted TTF to WOFF2 with ${woff2.length} bytes in ${elapsed.toFixed(2)}ms</p>`;
        if (autoDownloadCheckbox.checked)
          download(woff2, filename.replace(/\.ttf$/, ".woff2"));
      } catch (e) {
        console.error(e);
        alert("invalid TTF file");
        resultsDiv.innerHTML = `<p>Failed to convert TTF to WOFF2</p>`;
      }
    });
  });
});

woffToTtfButton.addEventListener("click", () => {
  upload((woff2, filename) => {
    resultsDiv.innerHTML = `<p>Converting WOFF2 with ${woff2.length} bytes to TTF...</p>`;
    waitUntilDOMUpdated(() => {
      try {
        const start = performance.now();
        const ttf = convert_woff2_to_ttf(woff2);
        const elapsed = performance.now() - start;
        resultsDiv.innerHTML += `<p>Converted WOFF2 to TTF with ${ttf.length} bytes in ${elapsed.toFixed(2)}ms</p>`;
        if (autoDownloadCheckbox.checked)
          download(ttf, filename.replace(/\.woff2$/, ".ttf"));
      } catch (e) {
        console.error(e);
        alert("invalid WOFF2 file");
        resultsDiv.innerHTML = `<p>Failed to convert WOFF2 to TTF</p>`;
      }
    });
  });
});
