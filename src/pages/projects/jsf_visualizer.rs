use jsfuckrs;
use leptos::*;

#[component]
pub fn JsfVisualizer(cx: Scope) -> impl IntoView {
    // let (src, set_src) = create_signal(cx, String::from("alert(1)"));
    let (compiled, set_compiled) = create_signal(cx, String::new());

    let compile = move |_| {
        let src = js_sys::eval(r#"ace.edit("editor").getValue()"#)
            .unwrap()
            .as_string()
            .unwrap();
        log::debug!("src: {}", &src);
        let result = jsfuckrs::lbp::compile(src);
        set_compiled.set(result);
    };

    let execute = move |_| {
        let y = js_sys::eval(&compiled()).unwrap();
        log::info!("{:?}", y);
    };

    view! {
        cx,
        <div class="px-[1rem] py-[1rem] max-w-3xl mx-auto">
            <h1 class="text-xl">"JSF*ck"</h1>
            <div><details class="mt-2 p-2">
                <summary>"Project Details"</summary>
                <p>
                    "The website "
                    <a class="link" href="https://esolangs.org/wiki/JSFuck" target="_blank" rel="noopener noreferrer">
                    "esolangs.org"
                    </a>
                    " describes JSF*ck is an "
                    <q>"esoteric subset of the JavaScript language that uses only six distinct characters in the source code."</q>
                </p>
                <div>
                    "The characters are: "
                    <ol class="leading-none list-none inline [&>li]:inline [&>li]:pr-1 p-0">
                        <li><code class="inline-code">"+"</code></li>
                        <li><code class="inline-code">"!"</code></li>
                        <li><code class="inline-code">"("</code></li>
                        <li><code class="inline-code">")"</code></li>
                        <li><code class="inline-code">"["</code></li>
                        <li><code class="inline-code">"]"</code></li>
                    </ol>
                </div>
                <p>
                    "I first encountered this programming language during a CTF event
                    and later came across this video from "
                    <a class="link" target="_blank" href="https://www.youtube.com/@LowByteProductions">
                        "Low Byte Productions"
                    </a>
                    " where he builds a compiler for this esolang in JavaScript."
                </p>
                <p>
                    "I ported his compiler implementation into Rust for the heck of it
                    with the goal of replicating his logic as close as possible."
                </p>
                <p>
                    "I then remembered the site "
                    <a class="link" href="https://jsfuck.com" target="_blank" rel="noopener noreferrer">
                        "jsfuck.com"
                    </a>
                    " and compared the output of my compiler with theirs (@aemkei on GitHub) and noticed
                    that the output of the two compilers was quite different for the same input."
                </p>
                <p>
                    "For the simple input of "
                    <strong><code class="inline-code">"alert(1)"</code></strong>
                    " the size difference between the two outputs is in the 10s of kilobytes."
                </p>
                <figure class="text-center">
                    <img src="/static/compiler_output_diff.png" alt="ls command output showing size difference between the two compilers' output" />
                    <figcaption>"Compiler output size diff"</figcaption>
                </figure>
                <p>
                    "I am now working on porting aemkei's compiler to Rust
                    and will update this page when I am done but for now you can enter 
                    in some JS source code and transpile it to jsf*ck using the 
                    Low Byte Productions implementation."
                </p>
                <hr class="mb-1"/>
            </details></div>

            <div id="editor" class="h-28 m-2">
                "alert(1)"
            </div>
            <div class="flex flex-row-reverse">
                <button
                    class="border rounded p-2 mx-2"
                    on:click=compile
                >
                    "Compile"
                </button>
            </div>
            // <input
            //     class="m-4 [writing-mode:horizontal-tb]"
            //     value=src
            //     on:change = move |ev| {
            //         let val = event_target_value(&ev);
            //         set_src.update(|v|*v = val);
            //     }
            // />
            <div class="h-80 overflow-auto border border-[var(--secondary)] rounded m-2">
                <code class="p-3">{compiled}</code>
            </div>
            <button
                class="border mx-2 rounded p-2 border-[var(--secondary)] relative float-right"
                on:click=execute
            >
                "Run This"
            </button>
            <style>
            r#"
            @keyframes open {
                0% {
                  opacity: 0
                }
                100% {
                  opacity: 1
                }
              }
              
              /* closing animation */
              @keyframes close {
                0% {
                  opacity: 1
                }
                100% {
                  opacity: 0
                }
              }
              
              details[open] summary~* {
                animation: open .5s;
              }
              
              /* closing class */
              details.closing summary~* {
                animation: close .5s;
              } 
            "#
            </style>
            <script defer>
                r#"
                const details = document.querySelector("details");
                details.addEventListener("click", function(e) {
                if (details.hasAttribute("open")) { // since it's not closed yet, it's open!
                    e.preventDefault(); // stop the default behavior, meaning - the hiding
                    details.classList.add("closing"); // add a class which apply the animation in CSS
                    setTimeout(() => { // only after the animation finishes, continue
                    details.removeAttribute("open"); // close the element
                    details.classList.remove("closing");
                    }, 400);
                }
                });
                "#
            </script>
            <script defer>
r#"
                var editor = ace.edit("editor");
                editor.setTheme("ace/theme/monokai");
                editor.session.setMode("ace/mode/javascript");

"#
            </script>
        </div>
    }
}
