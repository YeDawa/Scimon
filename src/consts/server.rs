pub struct Server;

impl Server {

    pub const LOGO_PNG: &str = "https://static.monlib.net/logo.png";

    pub const SOURCE_ROUTE: &str = "/__scimon/source.mon";
    pub const PARSE_ROUTE: &str = "/__scimon/parse.json";

    pub const ARCHIVE_ROUTE: &str = "/__scimon/archive";

    pub const SCRIPTS_ROUTE: &str = "/__scimon/scripts";
    pub const SCRIPT_ROUTE: &str = "/__scimon/script/";

    pub const CHECKSUM_HASH_ROUTE: &str = "/__scimon/checksum/";

    pub const ZIP_LIST_ROUTE: &str = "/__scimon/ziplist/";
    pub const ARCHIVE_LIST_ROUTE: &str = "/__scimon/archivelist";

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
            function refresh(){
                btn.innerHTML='<i data-lucide="'+(isDark()?'sun':'moon')+'"></i>';
                if(window.lucide)lucide.createIcons();
            }
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
            border-right:1px solid var(--border);display:flex;flex-direction:column;}
        .main{flex:1;min-width:0;box-sizing:border-box;padding:1.3rem 1.6rem;}
        .sidebar .item{display:flex;align-items:center;gap:.5rem;margin-top:1.2rem;
            padding:.5rem .7rem;border-radius:8px;background:var(--hover);color:var(--fg);}
        .sidebar .item:hover{text-decoration:none;}
        .sidebar .folders{display:flex;flex-direction:column;gap:.15rem;margin-top:1.2rem;}
        .sidebar .folders a{display:flex;align-items:center;gap:.5rem;padding:.4rem .6rem;
            border-radius:6px;color:var(--fg);font-size:.9rem;}
        .sidebar .folders a:hover{background:var(--hover);text-decoration:none;}
        .sidebar .folders a.active{background:var(--hover);font-weight:600;}
        .sidebar .separator{height:1px;background:var(--border);margin:.5rem 0;}
        .logo{align-self:center;}
        .brand-meta{align-self:center;display:flex;align-items:center;justify-content:center;gap:.35rem;margin-top:.5rem;font-size:.8rem;color:var(--muted);}
        .brand-meta a{display:inline-flex;align-items:center;color:var(--muted);}
        .brand-meta a:hover{color:var(--link);}
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
        .search{margin-top:1.5em;width:100%;max-width:340px;margin-bottom:1rem;padding:.5rem .75rem;
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
        .cs-modal{position:fixed;inset:0;background:rgba(0,0,0,.6);display:none;
            align-items:center;justify-content:center;z-index:1100;}
        .cs-modal.open{display:flex;}
        .cs-card{position:relative;background:var(--bg);color:var(--fg);border:1px solid var(--border);
            border-radius:12px;padding:1.5rem;width:min(560px,92vw);
            display:flex;flex-direction:column;gap:.9rem;box-shadow:0 10px 40px rgba(0,0,0,.4);}
        .cs-card h2{margin:0;font-size:1.1rem;}
        .cs-card .cs-row{display:flex;flex-direction:column;gap:.3rem;font-size:.9rem;}
        .cs-card select,.cs-card input{padding:.5rem .7rem;border:1px solid var(--border);
            border-radius:8px;background:var(--bg);color:var(--fg);font-size:.9rem;}
        .cs-card button{align-self:flex-start;padding:.5rem .9rem;border:1px solid var(--border);
            border-radius:8px;background:var(--hover);color:var(--fg);cursor:pointer;font-size:.9rem;}
        .cs-card code{word-break:break-all;font-family:ui-monospace,Consolas,monospace;color:var(--fg);}
        .cs-card .cs-close{position:absolute;top:.7rem;right:1rem;cursor:pointer;font-size:1.6rem;
            line-height:1;color:var(--muted);}
        .cs-card .cs-close:hover{color:var(--fg);}
        .cs-ok{color:#16a34a;font-weight:600;}
        .cs-bad{color:#dc2626;font-weight:600;}
        .theme-toggle{margin-top:auto;align-self:flex-start;display:flex;align-items:center;gap:.5rem;
            background:transparent;color:var(--fg);border:1px solid var(--border);border-radius:8px;
            padding:.45rem .7rem;cursor:pointer;font-size:1rem;line-height:1;}
        .theme-toggle:hover{background:var(--hover);}
        .theme-toggle svg{width:18px;height:18px;}
        #lb{position:fixed;inset:0;background:rgba(0,0,0,.85);display:none;
            align-items:center;justify-content:center;z-index:1000;}
        #lb.open{display:flex;}
        #lb .figure{margin:0;display:flex;flex-direction:column;align-items:center;gap:.7rem;}
        #lb .stage{display:flex;align-items:center;justify-content:center;}
        #lb img{max-width:90vw;max-height:82vh;border-radius:4px;
            background:var(--lb-img-bg);padding:var(--lb-img-pad);box-sizing:border-box;}
        #lb iframe{width:90vw;height:82vh;border:0;border-radius:4px;background:#fff;}
        #lb .epub-view{width:90vw;max-width:1000px;height:82vh;background:#fff;
            border-radius:4px;overflow:auto;}
        #lb .zip-list{width:90vw;max-width:1000px;max-height:82vh;overflow:auto;
            background:#111;color:#eee;border-radius:4px;padding:1rem 1.2rem;box-sizing:border-box;}
        #lb .zip-list ul{list-style:none;margin:0;padding:0;}
        #lb .zip-list li{display:flex;align-items:center;gap:.6rem;padding:.25rem 0;
            font-family:ui-monospace,Consolas,monospace;font-size:.9rem;}
        #lb .zip-list .zn{flex:1;overflow-wrap:anywhere;}
        #lb .zip-list .zs{color:#9aa0a6;white-space:nowrap;}
        #lb .zip-list .zip-dl{display:inline-block;margin-bottom:.8rem;padding:.35rem .8rem;
            border:1px solid #444;border-radius:6px;color:#6ea8fe;font-size:.85rem;}
        #lb .zip-list .zip-dl:hover{background:#1a1d24;text-decoration:none;}
        #lb .cm-box{width:90vw;max-width:1000px;}
        #lb .CodeMirror{width:100%;height:82vh;border-radius:6px;font-size:.9rem;}
        #lb .cm-box:not(:has(.CodeMirror)){background:#111;color:#eee;padding:1rem 1.2rem;
            border-radius:4px;max-height:82vh;overflow:auto;white-space:pre-wrap;
            word-break:break-all;font-family:ui-monospace,Consolas,monospace;}
        #lb .cm-tabwrap{width:90vw;max-width:1000px;display:flex;flex-direction:column;}
        #lb .cm-tabwrap .cm-box{width:100%;max-width:none;}
        #lb .cm-tabwrap .CodeMirror{height:76vh;}
        #lb .cm-tabs{display:flex;gap:.3rem;margin-bottom:.5rem;}
        #lb .cm-tab{padding:.4rem .9rem;border:1px solid #2b2f37;border-bottom:none;
            border-radius:6px 6px 0 0;background:#1a1d24;color:#9aa0a6;cursor:pointer;
            font-size:.85rem;font-family:ui-monospace,Consolas,monospace;}
        #lb .cm-tab:hover{color:#eee;}
        #lb .cm-tab.active{background:#111;color:#eee;}
        #lb .cap{color:#eee;font-size:.95rem;max-width:90vw;text-align:center;
            overflow-wrap:anywhere;}
        #lb .btn{position:absolute;color:#fff;cursor:pointer;user-select:none;
            font-size:2rem;padding:.4rem 1rem;opacity:.8;line-height:1;z-index:1002;}
        #lb .btn:hover{opacity:1;}
        #lb .close{top:1rem;right:1.5rem;font-size:2.4rem;}
        #lb .prev{left:.5rem;top:50%;transform:translateY(-50%);}
        #lb .next{right:.5rem;top:50%;transform:translateY(-50%);}
    "#;

    pub const SCIMON_MODE_JS: &'static str = r#"
        (function(){
            if(!window.CodeMirror||!CodeMirror.defineSimpleMode)return;
            CodeMirror.defineSimpleMode('scimon',{
                start:[
                    {regex:/@\w+/,token:'meta'},
                    {regex:/\b(downloads|commands|readme|ai)\b(?=\s*\{)/,token:'keyword'},
                    {regex:/\b(import|path|open|compress|copy|covers|qrcode|style|print|readme|math|server|as)\b/,token:'keyword'},
                    {regex:/![a-zA-Z_]+/,token:'atom'},
                    {regex:/"(?:[^"\\]|\\.)*"/,token:'string'},
                    {regex:/https?:\/\/\S+/,token:'link'},
                    {regex:/(?:^|\s)\/\/.*$/,token:'comment'},
                    {regex:/(?:^|\s)\/\*/,token:'comment',next:'blockComment'},
                    {regex:/[{}]/,token:'bracket'},
                    {regex:/>/,token:'operator'}
                ],
                blockComment:[
                    {regex:/.*?\*\//,token:'comment',next:'start'},
                    {regex:/.*/,token:'comment'}
                ]
            });
        })();
    "#;

    pub const CHECKSUM_JS: &'static str = r#"
        (function(){
            var modal=document.getElementById('cs-modal');
            if(!modal)return;
            var sel=document.getElementById('cs-file');
            var btn=document.getElementById('cs-compute');
            var res=document.getElementById('cs-result');
            var exp=document.getElementById('cs-expected');
            var status=document.getElementById('cs-status');
            function compare(){
                var c=(res.textContent||'').trim().toLowerCase();
                var e=(exp.value||'').trim().toLowerCase();
                if(!c||c==='—'||c==='…'||!e){status.textContent='';status.className='';return;}
                if(c===e){status.textContent='✓ Match';status.className='cs-ok';}
                else{status.textContent='✗ No match';status.className='cs-bad';}
            }
            function open(){modal.classList.add('open');}
            function close(){modal.classList.remove('open');}
            document.querySelectorAll('.cs-open').forEach(function(a){
                a.addEventListener('click',function(e){e.preventDefault();open();});
            });
            modal.addEventListener('click',function(e){
                if(e.target===modal||e.target.classList.contains('cs-close'))close();
            });
            document.addEventListener('keydown',function(e){
                if(e.key==='Escape'&&modal.classList.contains('open'))close();
            });
            if(btn)btn.addEventListener('click',function(){
                res.textContent='…';status.textContent='';
                fetch('/checksum/'+encodeURIComponent(sel.value))
                    .then(function(r){return r.text();})
                    .then(function(t){res.textContent=t.trim();compare();})
                    .catch(function(){res.textContent='error';});
            });
            if(exp)exp.addEventListener('input',compare);
        })();
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
            var book=null;
            var rendition=null;
            function destroyBook(){
                if(book){try{book.destroy();}catch(e){}}
                book=null;rendition=null;
            }
            function renderCode(box,url,name){
                fetch(url).then(function(r){return r.text();})
                    .then(function(t){
                        if(!window.CodeMirror){box.textContent=t;return;}
                        var isMon=/\.mon$/i.test(name);
                        var info=(!isMon&&CodeMirror.findModeByFileName)?CodeMirror.findModeByFileName(name):null;
                        var cm=CodeMirror(box,{
                            value:t,
                            readOnly:'nocursor',
                            lineNumbers:true,
                            lineWrapping:true,
                            theme:'material-darker',
                            mode:isMon?'scimon':(info?info.mode:null),
                            viewportMargin:Infinity
                        });
                        if(info&&CodeMirror.autoLoadMode)CodeMirror.autoLoadMode(cm,info.mode);
                    })
                    .catch(function(){box.textContent='Failed to load file.';});
            }
            function renderTabbed(stage,tabs){
                var wrap=document.createElement('div');
                wrap.className='cm-tabwrap';
                var bar=document.createElement('div');
                bar.className='cm-tabs';
                var box=document.createElement('div');
                box.className='cm-box';
                function select(idx){
                    [].forEach.call(bar.children,function(b,i){
                        b.classList.toggle('active',i===idx);
                    });
                    box.innerHTML='';
                    renderCode(box,tabs[idx].url,tabs[idx].label);
                }
                tabs.forEach(function(tab,i){
                    var b=document.createElement('button');
                    b.className='cm-tab';
                    b.textContent=tab.label;
                    b.addEventListener('click',function(){select(i);});
                    bar.appendChild(b);
                });
                wrap.appendChild(bar);
                wrap.appendChild(box);
                stage.appendChild(wrap);
                select(0);
            }
            function show(n){
                i=(n+links.length)%links.length;
                var link=links[i];
                var type=link.getAttribute('data-type');
                var href=link.getAttribute('href');
                destroyBook();
                stage.innerHTML='';
                if(type==='image'){
                    var img=document.createElement('img');
                    img.src=href;img.alt=link.textContent;
                    stage.appendChild(img);
                }else if(type==='pdf'){
                    var frame=document.createElement('iframe');
                    frame.src=href;
                    stage.appendChild(frame);
                }else if(type==='epub'){
                    var holder=document.createElement('div');
                    holder.className='epub-view';
                    stage.appendChild(holder);
                    if(window.ePub){
                        try{
                            book=ePub(href);
                            rendition=book.renderTo(holder,{
                                manager:'continuous',
                                flow:'scrolled',
                                width:'100%',
                                height:'100%',
                                allowScriptedContent:true
                            });
                            rendition.themes.font('system-ui, sans-serif');
                            rendition.display();
                        }catch(e){holder.textContent='Failed to open EPUB.';}
                    }else{
                        holder.textContent='EPUB reader failed to load.';
                    }
                }else if(type==='zip'){
                    var zbox=document.createElement('div');
                    zbox.className='zip-list';
                    zbox.textContent='Loading…';
                    stage.appendChild(zbox);
                    var listUrl=link.getAttribute('data-list')||('/__scimon/ziplist/'+href.replace(/^\//,''));
                    var dlUrl=link.getAttribute('data-download')||href;
                    fetch(listUrl)
                        .then(function(r){return r.ok?r.json():Promise.reject();})
                        .then(function(items){
                            zbox.textContent='';
                            var dl=document.createElement('a');
                            dl.className='zip-dl';dl.href=dlUrl;dl.setAttribute('download','');
                            dl.textContent='Download archive';
                            zbox.appendChild(dl);
                            if(!items.length){var em=document.createElement('div');em.textContent='Empty archive.';zbox.appendChild(em);return;}
                            var ul=document.createElement('ul');
                            items.forEach(function(it){
                                var li=document.createElement('li');
                                var ic=document.createElement('span');ic.className='icon';
                                ic.innerHTML='<i data-lucide="'+(it.dir?'folder':'file')+'"></i>';
                                var nm=document.createElement('span');nm.className='zn';nm.textContent=it.name;
                                var sz=document.createElement('span');sz.className='zs';sz.textContent=it.size||'';
                                li.appendChild(ic);li.appendChild(nm);li.appendChild(sz);
                                ul.appendChild(li);
                            });
                            zbox.appendChild(ul);
                            if(window.lucide)lucide.createIcons();
                        })
                        .catch(function(){zbox.textContent='Failed to read archive.';});
                }else{
                    var name=link.textContent.trim();
                    var parseUrl=link.getAttribute('data-parse');
                    if(parseUrl){
                        renderTabbed(stage,[
                            {label:name,url:href},
                            {label:'parse.json',url:parseUrl}
                        ]);
                    }else{
                        var box=document.createElement('div');
                        box.className='cm-box';
                        stage.appendChild(box);
                        renderCode(box,href,name);
                    }
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
            function close(){lb.classList.remove('open');destroyBook();stage.innerHTML='';}
            function nav(dir){
                if(links.length>1){show(i+dir);}
            }
            document.addEventListener('click',function(e){
                var a=e.target.closest?e.target.closest('a.lb'):null;
                if(a){e.preventDefault();open(a);}
            });
            lb.addEventListener('click',function(e){
                var t=e.target;
                if(t.classList.contains('next')){nav(1);}
                else if(t.classList.contains('prev')){nav(-1);}
                else if(t.classList.contains('close')||t===lb||t.classList.contains('stage')||t.classList.contains('figure')){close();}
            });
            document.addEventListener('keydown',function(e){
                if(!lb.classList.contains('open'))return;
                if(e.key==='Escape')close();
                else if(e.key==='ArrowRight')nav(1);
                else if(e.key==='ArrowLeft')nav(-1);
            });
        })();
    "#;

}