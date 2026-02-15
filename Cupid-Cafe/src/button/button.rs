use std::ptr::null;

use turbo::*;


#[turbo::serialize]
pub struct Button {
    pub hitbox: (f32, f32, f32, f32),
    pub text: String,
    pub hovered: bool,
    pub action: bool,
    pub hold: bool,
}

impl Button {
    pub fn new (text: &str, hitbox: (f32, f32, f32, f32), act: bool) -> Self {
        Self {
            hitbox, // x, y, w, h
            text: text.to_string(), // button text
            hovered: false, // hover state
            action: act, //checks if specific button was pressed or not
            hold: false,
        }
    }

    //draws the button onto the screen
    //will add a additional variable in parameter to see check if
    //the button thats being drawn is an ingredient or not
    //(ex. draw(&self, name: &str))
    pub fn draw(&self, noH: bool) {
        //draws button and highlighted button
        let highlight = format!("{}_highlight", &self.text);
        if noH {
            sprite!(&self.text, x = self.hitbox.0, y = self.hitbox.1);
        } else if !noH {
            
            match self.hovered {
                true => {
                    if self.text == "artist" {
                        sprite!(&highlight, x = self.hitbox.0 - 15.0, y = self.hitbox.1 - 1.0);
                    } else if self.text == "yes" {
                        sprite!(&highlight, x = self.hitbox.0 - 12.0, y = self.hitbox.1 - 5.0);
                    } else {
                        sprite!(&highlight, x = self.hitbox.0 - 1.0, y = self.hitbox.1 - 1.0);
                    }
                }
                false => sprite!(&self.text, x = self.hitbox.0, y = self.hitbox.1)
            };
        }
        
    }

    pub fn nonselect(&self) {
        let nonselect = format!("{}_nonselect", &self.text);
        // match self.hovered {
        //     true => sprite!(&highlight, x = self.hitbox.0 - 1, y = self.hitbox.1 - 1),
        //     false => sprite!(&self.text, x = self.hitbox.0, y = self.hitbox.1)
        // };
        sprite!(&nonselect, x = self.hitbox.0, y = self.hitbox.1);
    }
     pub fn tempDraw(&self, name: &str) {
        // Color references
        if name == "empty" {
            return;
        }
        let (c1, c2): (u32, u32) = match self.hovered {
            true => (0x323b42ff, 0xffffffff),
            false => (0xffffffff, 0x323b42ff)
        };
        // Calculate text offset for centering
        let (x, y) = 
            (self.hitbox.0 + (self.hitbox.2/2.0) - (self.text.len() as f32 * 2.5), 
            self.hitbox.1 + (self.hitbox.3/2.0) - 3.0);

        // Draw button
        if name == "no" {
            rect!(x = self.hitbox.0, y = self.hitbox.1, w = self.hitbox.2, h = self.hitbox.3, color = 0x22406eff);
        } else {
            rect!(x = self.hitbox.0, y = self.hitbox.1, w = self.hitbox.2, h = self.hitbox.3, color = c1);
        }
    }

    //checks if the mouse is hovering the button or not
    pub fn check(&mut self, mut select: (f32,f32)) -> (f32,f32){
        //gets the mouses world space position (its x and y on screen)
        let m = pointer::world();
        let(mx, my) = m.xy();
        let x = mx as f32;
        let y = my as f32;
        if self.hover(self.hitbox, x, y) {
            if m.just_released() {
                self.action = true; // Call function local to button
                return (self.hitbox.0, self.hitbox.1);
            } else {
                self.action = false;
                return (self.hitbox.0, self.hitbox.1);
            } 
        } else {
            return select;
        }
    }

    pub fn hover(&mut self, hitbox: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
        if x >= hitbox.0 && x <= hitbox.0 + hitbox.2
        && y >= hitbox.1 && y <= hitbox.1 + hitbox.3 {
            self.hovered = true;
            return true;
        } else {
            self.hovered = false;
            return false;
        }
    }
}