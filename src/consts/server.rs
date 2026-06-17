pub struct Server;

impl Server {

    pub const LOGO_PNG: &str = "https://static.monlib.net/scimon.png";

    pub const SOURCE_ROUTE: &str = "/__scimon/source.mon";

    pub const THEME_EARLY: &'static str = "<script>(function(){try{\
        var t=localStorage.getItem('scimon-theme');\
        if(t)document.documentElement.setAttribute('data-theme',t);\
        }catch(e){}})();</script>";

    pub const THEME_TOGGLE: &'static str =
        "<button id=\"themeBtn\" class=\"theme-toggle\" aria-label=\"Toggle theme\"></button>";

    pub const THEME_JS: &'static str = r#"
        (function(){
            var root=document.documentElement;
            var btn=document.getElementById('themeBtn');
            if(!btn)return;
            function isDark(){
                var t=root.getAttribute('data-theme');
                if(t)return t==='dark';
                return window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches;
            }
            function refresh(){btn.textContent=isDark()?'☀️':'🌙';}
            refresh();
            btn.addEventListener('click',function(){
                var next=isDark()?'light':'dark';
                root.setAttribute('data-theme',next);
                try{localStorage.setItem('scimon-theme',next);}catch(e){}
                refresh();
            });
        })();
    "#;

    pub const STYLE: &'static str = r#"
        :root{--bg:#fff;--fg:#1a1a1a;--link:#2563eb;--muted:#777;--logo-filter:none;
            --lb-img-bg:transparent;--lb-img-pad:0;}
        @media (prefers-color-scheme:dark){
            :root{--bg:#0f1115;--fg:#e6e6e6;--link:#6ea8fe;--muted:#9aa0a6;--logo-filter:brightness(1.7);
                --lb-img-bg:#fff;--lb-img-pad:8px;}
        }
        :root[data-theme="light"]{--bg:#fff;--fg:#1a1a1a;--link:#2563eb;--muted:#777;--logo-filter:none;
            --lb-img-bg:transparent;--lb-img-pad:0;}
        :root[data-theme="dark"]{--bg:#0f1115;--fg:#e6e6e6;--link:#6ea8fe;--muted:#9aa0a6;--logo-filter:brightness(1.7);
            --lb-img-bg:#fff;--lb-img-pad:8px;}
        body{font-family:system-ui,sans-serif;margin:2rem;background:var(--bg);color:var(--fg);}
        .logo{display:inline-block;margin-bottom:1.2rem;}
        .logo img{display:block;filter:var(--logo-filter);}
        h1{font-size:1.2rem;}
        ul{list-style:none;padding:0;}
        li{padding:.2rem 0;}
        a{text-decoration:none;color:var(--link);}
        a:hover{text-decoration:underline;}
        .source{margin:.2rem 0 1rem;font-size:.9rem;}
        .theme-toggle{position:fixed;top:1rem;right:1rem;background:transparent;
            border:1px solid var(--muted);color:var(--fg);border-radius:6px;
            padding:.3rem .55rem;cursor:pointer;font-size:1rem;line-height:1;}
        .theme-toggle:hover{border-color:var(--fg);}
        #lb{position:fixed;inset:0;background:rgba(0,0,0,.85);display:none;
            align-items:center;justify-content:center;z-index:1000;}
        #lb.open{display:flex;}
        #lb .figure{margin:0;display:flex;flex-direction:column;align-items:center;gap:.7rem;}
        #lb .stage{display:flex;align-items:center;justify-content:center;}
        #lb img{max-width:90vw;max-height:82vh;border-radius:4px;
            background:var(--lb-img-bg);padding:var(--lb-img-pad);box-sizing:border-box;}
        #lb iframe{width:90vw;height:82vh;border:0;border-radius:4px;background:#fff;}
        #lb pre{margin:0;background:#111;color:#eee;padding:1rem 1.2rem;border-radius:4px;
            max-width:90vw;max-height:82vh;overflow:auto;white-space:pre-wrap;
            word-break:break-all;font-family:ui-monospace,Consolas,monospace;font-size:.9rem;}
        #lb .cap{color:#eee;font-size:.95rem;max-width:90vw;text-align:center;
            overflow-wrap:anywhere;}
        #lb .btn{position:absolute;color:#fff;cursor:pointer;user-select:none;
            font-size:2rem;padding:.4rem 1rem;opacity:.8;line-height:1;}
        #lb .btn:hover{opacity:1;}
        #lb .close{top:1rem;right:1.5rem;font-size:2.4rem;}
        #lb .prev{left:.5rem;top:50%;transform:translateY(-50%);}
        #lb .next{right:.5rem;top:50%;transform:translateY(-50%);}
    "#;

    pub const LIGHTBOX_HTML: &'static str = "<div id=\"lb\">\
        <span class=\"btn close\">&times;</span>\
        <span class=\"btn prev\">&#10094;</span>\
        <figure class=\"figure\"><div class=\"stage\"></div><figcaption class=\"cap\"></figcaption></figure>\
        <span class=\"btn next\">&#10095;</span></div>";

    pub const LIGHTBOX_JS: &'static str = r#"
        (function(){
            var lb=document.getElementById('lb');
            var stage=lb.querySelector('.stage');
            var cap=lb.querySelector('.cap');
            var links=[].slice.call(document.querySelectorAll('a.lb'));
            var i=0;
            if(links.length<2){
                lb.querySelector('.prev').style.display='none';
                lb.querySelector('.next').style.display='none';
            }
            function show(n){
                i=(n+links.length)%links.length;
                var link=links[i];
                var type=link.getAttribute('data-type');
                var href=link.getAttribute('href');
                stage.innerHTML='';
                if(type==='image'){
                    var img=document.createElement('img');
                    img.src=href;img.alt=link.textContent;
                    stage.appendChild(img);
                }else if(type==='pdf'){
                    var frame=document.createElement('iframe');
                    frame.src=href;
                    stage.appendChild(frame);
                }else{
                    var pre=document.createElement('pre');
                    pre.textContent='Loading…';
                    stage.appendChild(pre);
                    fetch(href).then(function(r){return r.text();})
                        .then(function(t){pre.textContent=t;})
                        .catch(function(){pre.textContent='Failed to load file.';});
                }
                cap.textContent=link.textContent;
            }
            function open(n){show(n);lb.classList.add('open');}
            function close(){lb.classList.remove('open');stage.innerHTML='';}
            links.forEach(function(a,idx){
                a.addEventListener('click',function(e){e.preventDefault();open(idx);});
            });
            lb.addEventListener('click',function(e){
                var t=e.target;
                if(t.classList.contains('next')){show(i+1);}
                else if(t.classList.contains('prev')){show(i-1);}
                else if(t.classList.contains('close')||t===lb||t.classList.contains('stage')||t.classList.contains('figure')){close();}
            });
            document.addEventListener('keydown',function(e){
                if(!lb.classList.contains('open'))return;
                if(e.key==='Escape')close();
                else if(e.key==='ArrowRight'&&links.length>1)show(i+1);
                else if(e.key==='ArrowLeft'&&links.length>1)show(i-1);
            });
        })();
    "#;

}