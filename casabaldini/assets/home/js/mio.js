
	// mio.js - Versione Autonoma per Dioxus
(function() {
    console.log("🎬 Regista SliderPro attivo e in attesa...");

    const avviaSlider = () => {
        const elemento = $('#example1');
        // Controlliamo che l'elemento esista e che non sia già stato inizializzato
        if (elemento.length && typeof $.fn.sliderPro !== 'undefined' && !elemento.hasClass('sp-initialized')) {
            console.log("🎯 Bersaglio trovato! Inizializzazione in corso...");
            elemento.sliderPro({
                width: 960,
                height: 500,
                arrows: true,
                buttons: false,
                waitForLayers: true,
                autoplay: true,
                autoHeight: true,
                imageScaleMode: 'contain'
            });
            // Aggiungiamo una classe per evitare doppie inizializzazioni
            elemento.addClass('sp-initialized');
        }
    };

    // Osserviamo il DOM: ogni volta che Dioxus aggiunge qualcosa, controlliamo se è lo slider
    const observer = new MutationObserver((mutations) => {
        avviaSlider();
    });

    observer.observe(document.body, {
        childList: true,
        subtree: true
    });

    // Prova anche al caricamento iniziale
    $(document).ready(avviaSlider);
})();