from django.shortcuts import get_object_or_404, render
from django.views.decorators.clickjacking import xframe_options_exempt

from .models import Food, Links, Menuweb
from .models import Entries
from .models import Slider

def home(request):
    luogo = "index"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.objects.filter(codice=luogo)
    links = Links.objects.all()[:]
    #slider = Connect.slider("", "mugello")
    #luogo = request.GET.get('luogo')
    carousel = "slide"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider, "links": links, "luogo": luogo, "carousel":carousel}
   
    return render(request, "beb/index.html", context)

def index(request):
    luogo = "index"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.objects.filter(codice=luogo)
    links = Links.objects.all()[:]
    #slider = Connect.slider("", "mugello")
    #luogo = request.GET.get('luogo')
    carousel = "slide"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider, "links": links, "luogo": luogo, "carousel":carousel}
   
    return render(request, "beb/index.html", context)


def detail(request, author_id):
    entries = get_object_or_404(Entries, pk=author_id)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    context = {"entries": entries, "menuweb": menuweb}
    return render(request, "beb/detail.html", context)


def slide(request):
    luogo = request.GET.get('luogo')
    ##slider = Slider.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    camere = Slider.objects.filter(codice="camere")[:]
    #slider = Connect.slider("", "mugello")
    context = {"slider": slider, "luogo": luogo, "camere": camere}
    #return render(request, "beb/nivo.html", context)
    return render(request, "beb/home/home.html", context)

def slide2(request):
    luogo = request.GET.get('luogo')
    ##slider = Slider.objects.all()[:]
    slider = Slider.objects.filter(codice2=luogo)[:]
    camere = Slider.objects.filter(codice2="camere")[:]
    #slider = Connect.slider("", "mugello")
    context = {"slider": slider, "luogo": luogo, "camere": camere}
    #return render(request, "beb/nivo.html", context)
    return render(request, "beb/home/home.html", context)


def blueslider(request):
    luogo = request.GET.get('luogo')
    ##slider = Slider.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    camere = Slider.objects.filter(codice="camere")[:]
    #slider = Connect.slider("", "mugello")
    context = {"slider": slider, "luogo": luogo, "camere": camere}
    #return render(request, "beb/nivo.html", context)
    return render(request, "beb/Blue-Slider-master/index.html", context)


def blueslider2(request):
    luogo = request.GET.get('luogo')
    ##slider = Slider.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    camere = Slider.objects.filter(codice2="camere")[:]
    #slider = Connect.slider("", "mugello")
    context = {"slider": slider, "luogo": luogo, "camere": camere}
    #return render(request, "beb/nivo.html", context)
    return render(request, "beb/Blue-Slider-master/index2.html", context)


def carousel(request):
    luogo = request.GET.get('luogo')
    ##slider = Slider.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    
    #slider = Connect.slider("", "mugello")
    context = {"slider": slider, "luogo": luogo}
    #return render(request, "beb/nivo.html", context)
    return render(request, "beb/carousel.html", context)


def camere__1(request):
    luogo = request.GET.get('luogo')
    ##slider = Slider.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    carousel = "carousel"
    links = Links.objects.all()[:]
    context = {"slider": slider, "luogo": luogo, "carousel": carousel,  "links": links}
   
    #return render(request, "beb/nivo.html", context)
    return render(request, "beb/index.html", context)


def menu(request):
        
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    context = {"menuweb": menuweb, "submenu": submenu}
    return render(request, "beb/AceMenu.html", context)


def camere(request):
    #luogo = request.GET.get('luogo')
    luogo = "camere"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.objects.filter(codice2=luogo)[:]
    links = Links.objects.all()[:]
    carousel = "slide2"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider,   "links": links,"luogo": luogo, "carousel": carousel}
    return render(request, "beb/index.html", context)


def camere_frame(request):
    #luogo = request.GET.get('luogo')
    luogo = "camere"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.objects.filter(codice2=luogo)[:]
    links = Links.objects.all()[:]
    carousel = "carousel"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider,   "links": links,"luogo": luogo, "carousel": carousel}
    return render(request, "beb/carousel.html", context)


def lasala(request):
    #luogo = request.GET.get('luogo')
    luogo = "lasala"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.objects.filter(codice2=luogo)[:]
    links = Links.objects.all()[:]
    carousel = "slide2"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider,  "links": links, "luogo": luogo, "carousel": carousel}
    return render(request, "beb/index.html", context)


def lasala_frame(request):
    #luogo = request.GET.get('luogo')
    luogo = "lasala"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.object
#from Connect import Connects.filter(codice=luogo)[:]
    links = Links.objects.all()[:]
    carousel = "slide2"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider,  "links": links, "luogo": luogo, "carousel": carousel}
    return render(request, "beb/carousel.html", context)


def ilpaese(request):
    #luogo = request.GET.get('luogo')
    luogo = "ilpaese"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    slider = Slider.objects.filter(codice2=luogo)[:]
    links = Links.objects.all()[:]
    carousel = "slide2"
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "slider": slider,  "links": links, "luogo": luogo, "carousel": carousel}
    return render(request, "beb/index.html", context)


def linkutili(request):
    #luogo = request.GET.get('luogo')
    luogo = "mugello"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    links = Links.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "links": links,  "luogo": luogo, "slider": slider}
    return render(request, "beb/linkutili.html", context)


def modal(request):
    #luogo = request.GET.get('luogo')
    luogo = "mugello"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    links = Links.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "links": links,  "luogo": luogo, "slider": slider}
    return render(request, "beb/popin2.html", context)

def prenotazioni(request):
    #luogo = request.GET.get('luogo')
    luogo = "mugello"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    links = Links.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "links": links,  "luogo": luogo, "slider": slider}
    return render(request, "beb/prenotazioni.html", context)

def dovemangiare(request):
    #luogo = request.GET.get('luogo')
    luogo = "mugello"
    entries = Entries.objects.filter(slug=luogo)
    menuweb = Menuweb.objects.filter(livello=2, attivo=1).order_by('ordine' , 'codice')
    submenu = Menuweb.objects.filter(livello=3, attivo=1).order_by('ordine', 'radice')
    links = Links.objects.all()[:]
    foods = Food.objects.all()[:]
    slider = Slider.objects.filter(codice=luogo)[:]
    context = {"entries": entries, "menuweb": menuweb, "submenu": submenu, "links": links, "foods": foods, "luogo": luogo, "slider": slider}
    return render(request, "beb/dovemangiare.html", context)