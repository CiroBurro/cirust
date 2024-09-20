let espressione = "";

document.querySelectorAll(".tasto").forEach(tasto => {
  tasto.addEventListener("click", () => {
    const valore = tasto.getAttribute("data-valore");
    espressione += valore;
    aggiornaSchermo();
  });
});

document.querySelector(".reset").addEventListener("click", () => {
  espressione = "";
  aggiornaSchermo();
});


document.querySelector(".uguale").addEventListener("click", async () => {
  try {
    const risultato = await window.__TAURI__.invoke("risolvi_espressione", { testo: espressione });
    console.log("Risultato ricevuto:", risultato);
    espressione = risultato;
    aggiornaSchermo();
  } catch (errore) {
    console.error("Errore nel calcolo:", errore);
    espressione = "Errore: " + errore;
    aggiornaSchermo();
  }
});

function aggiornaSchermo() {
  document.querySelector(".schermo").innerText = espressione;
}