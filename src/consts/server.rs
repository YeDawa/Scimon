pub struct Server;

impl Server {

    pub const LOGO_PNG: &str = "https://static.monlib.net/scimon.png";

    pub const SOURCE_ROUTE: &str = "/__scimon/source.mon";

    pub const ARCHIVE_ROUTE: &str = "/__scimon/archive";

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
            --lb-img-bg:transparent;--lb-img-pad:0;--border:#e5e7eb;--hover:#f5f6f8;}
        @media (prefers-color-scheme:dark){
            :root{--bg:#0f1115;--fg:#e6e6e6;--link:#6ea8fe;--muted:#9aa0a6;--logo-filter:brightness(1.7);
                --lb-img-bg:#fff;--lb-img-pad:8px;--border:#272b33;--hover:#1a1d24;}
        }
        :root[data-theme="light"]{--bg:#fff;--fg:#1a1a1a;--link:#2563eb;--muted:#777;--logo-filter:none;
            --lb-img-bg:transparent;--lb-img-pad:0;--border:#e5e7eb;--hover:#f5f6f8;}
        :root[data-theme="dark"]{--bg:#0f1115;--fg:#e6e6e6;--link:#6ea8fe;--muted:#9aa0a6;--logo-filter:brightness(1.7);
            --lb-img-bg:#fff;--lb-img-pad:8px;--border:#272b33;--hover:#1a1d24;}
        body{font-family:system-ui,sans-serif;margin:0;background:var(--bg);color:var(--fg);}
        .layout{display:flex;min-height:100vh;}
        .sidebar{width:240px;flex:none;box-sizing:border-box;padding:1.3rem;
            border-right:1px solid var(--border);}
        .main{flex:1;min-width:0;box-sizing:border-box;padding:1.3rem 1.6rem;}
        .sidebar .item{display:flex;align-items:center;gap:.5rem;margin-top:1.2rem;
            padding:.5rem .7rem;border-radius:8px;background:var(--hover);color:var(--fg);}
        .sidebar .item:hover{text-decoration:none;}
        .sidebar .folders{display:flex;flex-direction:column;gap:.15rem;margin-top:1.2rem;}
        .sidebar .folders a{display:flex;align-items:center;gap:.5rem;padding:.4rem .6rem;
            border-radius:6px;color:var(--fg);font-size:.9rem;}
        .sidebar .folders a:hover{background:var(--hover);text-decoration:none;}
        .sidebar .folders a.active{background:var(--hover);font-weight:600;}
        .logo{display:inline-block;}
        .logo img{display:block;filter:var(--logo-filter);}
        h1{font-size:1.2rem;}
        ul{list-style:none;padding:0;}
        li{display:flex;align-items:center;padding:.2rem 0;}
        .icon{display:inline-flex;width:1.5em;align-items:center;flex:none;}
        .icon svg{width:18px;height:18px;}
        a{text-decoration:none;color:var(--link);}
        a:hover{text-decoration:underline;}
        .source{margin:.2rem 0 1rem;font-size:.9rem;}
        .source a{display:inline-flex;align-items:center;}
        .search{width:100%;max-width:340px;margin-bottom:1rem;padding:.5rem .75rem;
            border:1px solid var(--border);border-radius:8px;background:var(--bg);
            color:var(--fg);font-size:.9rem;box-sizing:border-box;}
        .search:focus{outline:none;border-color:var(--link);}
        table.files{width:100%;border-collapse:collapse;font-size:.9rem;}
        table.files thead th{text-align:left;padding:.6rem .8rem;border-bottom:1px solid var(--border);
            color:var(--muted);font-weight:600;cursor:pointer;user-select:none;white-space:nowrap;}
        table.files thead th:hover{color:var(--fg);}
        table.files th.num,table.files td.num{text-align:right;}
        table.files tbody td{padding:.5rem .8rem;border-bottom:1px solid var(--border);vertical-align:middle;}
        table.files tbody tr:hover{background:var(--hover);}
        table.files td.name a{display:inline-flex;align-items:center;gap:.5rem;}
        table.files td.meta{color:var(--muted);white-space:nowrap;}
        table.files .arrow{font-size:.7em;opacity:.6;margin-left:.25rem;}
        .theme-toggle{position:fixed;top:1rem;right:1rem;background:transparent;
            color:var(--fg);border:none;border-radius:6px;
            padding:.3rem .55rem;cursor:pointer;font-size:1rem;line-height:1;}
        .theme-toggle:hover{background:var(--fg);}
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

    pub const SEARCH_JS: &'static str = r#"
        (function(){
            var input=document.getElementById('search');
            var table=document.querySelector('table.files');
            if(!input||!table)return;
            var tbody=table.querySelector('tbody');
            var original=tbody.innerHTML;
            var files=window.__scimonFiles||[];
            function esc(s){return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');}
            function row(f){
                var attrs=f.t?(' class="lb" data-type="'+f.t+'"'):'';
                return '<tr data-dir="0" data-name="'+esc(f.p)+'" data-size="'+f.size+'" data-mtime="'+f.mtime+'">'+
                    '<td class="name"><a'+attrs+' href="'+esc(f.h)+'"><span class="icon"><i data-lucide="'+f.i+'"></i></span> '+esc(f.p)+'</a></td>'+
                    '<td class="meta">'+esc(f.m)+'</td><td class="meta num">'+esc(f.s)+'</td></tr>';
            }
            function refresh(){ if(window.lucide)lucide.createIcons(); }
            input.addEventListener('input',function(){
                var q=input.value.toLowerCase().trim();
                if(!q){ tbody.innerHTML=original; refresh(); return; }
                var matches=files.filter(function(f){return f.p.toLowerCase().indexOf(q)>=0;});
                tbody.innerHTML=matches.length?matches.map(row).join(''):'<tr><td colspan="3" class="meta">No matches.</td></tr>';
                refresh();
            });
        })();
    "#;

    pub const TABLE_JS: &'static str = r#"
        (function(){
            var table=document.querySelector('table.files');
            if(!table)return;
            var tbody=table.querySelector('tbody');
            table.querySelectorAll('th[data-key]').forEach(function(th){
                th.addEventListener('click',function(){
                    var key=th.getAttribute('data-key');
                    var asc=th.getAttribute('data-asc')!=='true';
                    table.querySelectorAll('th[data-key]').forEach(function(h){
                        h.removeAttribute('data-asc');
                        var a=h.querySelector('.arrow'); if(a)a.textContent='';
                    });
                    th.setAttribute('data-asc',asc);
                    var arrow=th.querySelector('.arrow'); if(arrow)arrow.textContent=asc?'▲':'▼';
                    var rows=[].slice.call(tbody.querySelectorAll('tr'));
                    rows.sort(function(a,b){
                        var dir=a.getAttribute('data-dir')==='1', db=b.getAttribute('data-dir')==='1';
                        if(dir!==db)return dir?-1:1;
                        var x=a.getAttribute('data-'+key), y=b.getAttribute('data-'+key);
                        if(key==='name'){x=x.toLowerCase();y=y.toLowerCase();return asc?(x<y?-1:x>y?1:0):(x>y?-1:x<y?1:0);}
                        x=parseFloat(x)||0;y=parseFloat(y)||0;return asc?x-y:y-x;
                    });
                    rows.forEach(function(r){tbody.appendChild(r);});
                });
            });
        })();
    "#;

    pub const LIGHTBOX_HTML: &'static str = "<div id=\"lb\">\
        <span class=\"btn close\">&times;</span>\
        <span class=\"btn prev\">&#10094;</span>\
        <figure class=\"figure\"><div class=\"stage\"></div><figcaption class=\"cap\"></figcaption></figure>\
        <span class=\"btn next\">&#10095;</span></div>";

    pub const LIGHTBOX_JS: &'static str = r#"
        (function(){
            var lb=document.getElementById('lb');
            if(!lb)return;
            var stage=lb.querySelector('.stage');
            var cap=lb.querySelector('.cap');
            var prev=lb.querySelector('.prev');
            var next=lb.querySelector('.next');
            var links=[];
            var i=0;
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
                cap.textContent=link.textContent.trim();
            }
            function open(el){
                // Snapshot the currently visible lightbox links for navigation.
                links=[].slice.call(document.querySelectorAll('a.lb')).filter(function(a){return a.offsetParent!==null;});
                i=links.indexOf(el);
                if(i<0){links=[el];i=0;}
                var multi=links.length>1;
                prev.style.display=multi?'':'none';
                next.style.display=multi?'':'none';
                show(i);
                lb.classList.add('open');
            }
            function close(){lb.classList.remove('open');stage.innerHTML='';}
            document.addEventListener('click',function(e){
                var a=e.target.closest?e.target.closest('a.lb'):null;
                if(a){e.preventDefault();open(a);}
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