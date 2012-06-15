#[doc="Applies style to boxes."]
import dom::base::{HTMLImageElement, Element, Node};
import dom::rcu::reader_methods;
import image::base::load;
import layout::base::*;
import style::style_methods;

impl ApplyStyleBoxMethods for @Box {
    fn apply_style_for_subtree() {
        self.apply_style();
        for btree.each_child(self) {
            |child|
            child.apply_style_for_subtree();
        }
    }

    #[doc="Applies CSS style."]
    fn apply_style() {
        // Right now, we only handle images.
        self.node.rd {
            |node|
            alt node.kind {
              ~Element(element) {
                let style = self.node.get_computed_style();

                self.appearance.background_color = some(style.back_color);

                alt element.kind {
                  ~HTMLImageElement(*) {
                    alt element.get_attr("src") {
                      some(url) {
                        // FIXME: Some sort of BASE HREF support!
                        // FIXME: Parse URLs!
                        // FIXME: Don't load synchronously!
                        #debug("loading image from %s", url);
                        let image = @load(url);
                        self.appearance.background_image = some(image);
                      }
                      none {
                        /* Ignore. */
                      }
                    }
                  }
                  _ { /* Ignore. */ }
                }
              }
              _ { /* Ignore. */ }
            }
        }
    }
}

