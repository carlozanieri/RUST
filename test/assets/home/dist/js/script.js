$(document).ready(function() {
	$('.carousel').carousel({
		width: 960,
		height: 500,
		arrows: true,
		buttons: false,
		waitForLayers: true,
		thumbnailWidth: 200,
		thumbnailHeight: 100,
		thumbnailPointer: true,
		autoplay: true,
		autoScaleLayers: true,
		breakpoints: {
			500: {
				thumbnailWidth: 120,
				thumbnailHeight: 50
			}
		}
	});
});