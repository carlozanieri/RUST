use crate::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    pub m: Menus,
    pub subitems: Vec<Submenus>,
}
pub fn NavItem(props: NavItemProps) -> Element {
    let mut is_open = use_signal(|| false);
    let route: Route = use_route();
    
    // 1. Normalizziamo il path corrente (es. "/casabaldini")
    let current_path = route.to_string();

    // 2. Funzione di utilità per confrontare i link in modo flessibile
    let check_active = |link: &str| {
        // Rimuoviamo gli slash iniziali da entrambi per un confronto pulito
        let clean_current = current_path.trim_start_matches('/');
        let clean_link = link.trim_start_matches('/');
        !clean_link.is_empty() && clean_current == clean_link
    };

    // 3. Logica per il Titolo Principale (Padre)
    let is_parent_active = props.subitems.iter().any(|s| check_active(&s.link));
    
    let parent_style = if is_parent_active {
        "text-decoration: none; color: #ff0000; font-weight: bold;"
    } else {
        "text-decoration: none; color: inherit; font-weight: bold;"
    };

    rsx! {
        li {
            style: "position: relative; list-style: none; display: inline-block; margin-right: 20px;",
            onmouseenter: move |_| is_open.set(true),
            onmouseleave: move |_| is_open.set(false),
            
            a { 
                href: "javascript:;", 
                style: "{parent_style}",
                "{props.m.titolo}" 
            }

            if is_open() && !props.subitems.is_empty() {
                ul { 
                    style: "position: absolute; background: white; border: 1px solid #ccc; z-index: 100; padding: 10px; margin: 0; min-width: 150px;",
                    for s in props.subitems.iter() {
                        li { 
                            key: "{s.id}",
                            style: "list-style: none; padding: 5px 0;",
                            a { 
                                href: s.link.clone(),
                                style: if check_active(&s.link) { 
                                    "text-decoration: none; color: #ff0000; display: block; font-weight: bold;" 
                                } else { 
                                    "text-decoration: none; color: #333; display: block;" 
                                },
                                "{s.titolo}" 
                            }
                        }
                    }
                }
            }
        }
    }
}