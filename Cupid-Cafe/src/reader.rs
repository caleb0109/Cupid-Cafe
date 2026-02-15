use std::string;

use turbo::{text::Text, utils::color, *};
static SCRIPT_PATH: &str = std::include_str!("script");
use crate::button::button::Button;


#[turbo::serialize]

//script reader line
//data reader line
//current line for script
//current line for data
//current Crime we are at (current level, current day, same shit)
//current Map we are on      ^
pub struct Reader {
    pub sLines: Vec<String>,
    pub current_line_s: usize,
    pub speaking: bool,
    pub newSpeakSet: bool,
    pub speakingProfile: String,
    pub cupidTalk: bool,
    pub npcName: String,
    pub posterPos: Tween<f32>,

    pub posterUp: bool,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            sLines: SCRIPT_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            current_line_s: 0,
            speaking: false,
            newSpeakSet: false,
            speakingProfile: "".to_string(),
            cupidTalk: false,
            npcName: "".to_string(),
            posterPos: Tween::new(-500.0),

            posterUp: false,
        }
    }


    //File reader set up
    //(number)(name)(number)
    //(which client)(npc talking)(npc progression)

    pub fn drawText(&mut self, start: &String, whoTalking: &String) {        
        let text ;
        
        if !self.newSpeakSet {
            match start.as_str() {
                "intro" => {
                    let n = self.sLines.iter().position(|line| line == "intro");
                    self.current_line_s = n.unwrap_or(0) + 1;
                    self.npcName = "Client".to_string();
                }
                "2client1" => {
                    let n = self.sLines.iter().position(|line| line == "2client1");
                    self.current_line_s = n.unwrap_or(0) + 1;
                    self.npcName = "Client".to_string();
                }
                _=> {
                    let n = self.sLines.iter().position(|line| line == start);
                    self.current_line_s = n.unwrap_or(0) + 1;
                }
            }
            self.newSpeakSet = true;
        }
        
        text = &self.sLines[self.current_line_s];
        
            
        if self.speaking == true {
            match whoTalking.as_str() {
                "artist" => {
                    sprite!(&self.speakingProfile, x = 338, y = 10);
                    self.npcName = "Jamie".to_string();
                    rect!(x = 205, y = 350, w = 80, h = 40,
                        color = 0x8072b7FF, 
                        border_size = 4, 
                        border_color = 0xa088ccff,
                        border_radius = 4
                    );
                    text!("{}", self.npcName;
                        x = 217,
                        y = 364, 
                        font = "TENPIXELS", 
                        color = 0xbdc8f4ff,
                    );
                } 
                "performative" => {
                    sprite!(&self.speakingProfile, x = 338, y = 10);
                    self.npcName = "Will".to_string();
                    rect!(x = 205, y = 350, w = 80, h = 40,
                        color = 0x52783dFF, 
                        border_size = 4, 
                        border_color = 0x7d9e61ff,
                        border_radius = 4
                    );
                    text!("{}", self.npcName;
                        x = 217,
                        y = 364, 
                        font = "TENPIXELS", 
                        color = 0xeddcb8ff,
                    );
                }
                "barista" => {
                    sprite!(&self.speakingProfile, x = 338, y = 10);
                    self.npcName = "Rae".to_string();
                    rect!(x = 205, y = 350, w = 80, h = 40,
                        color = 0x274d64FF, 
                        border_size = 4, 
                        border_color = 0x81dfd0ff,
                        border_radius = 4
                    );
                    text!("{}", self.npcName;
                        x = 217,
                        y = 364, 
                        font = "TENPIXELS", 
                        color = 0x81dfd0ff,
                    );
                }
                "bingleFart" => {
                    sprite!(&self.speakingProfile, x = 338, y = 165);
                    self.npcName = "Mro".to_string();
                    rect!(x = 205, y = 350, w = 80, h = 40,
                        color = 0xe57248FF, 
                        border_size = 4, 
                        border_color = 0xf5a764ff,
                        border_radius = 4
                    );
                    text!("{}", self.npcName;
                        x = 217,
                        y = 364, 
                        font = "TENPIXELS", 
                        color = 0xffec8aff,
                    );
                }
                _=> {
                    self.npcName = "Client".to_string();
                    rect!(x = 205, y = 350, w = 80, h = 40,
                        color = 0x274d64FF, 
                        border_size = 4, 
                        border_color = 0xffec8aff,
                        border_radius = 4
                    );
                    text!("{}", self.npcName;
                        x = 217,
                        y = 364, 
                        font = "TENPIXELS", 
                        color = 0x81dfd0ff,
                    );
                }
            }
            sprite!("cupid", x = 0, y = 10);
            if self.cupidTalk {
                rect!(x = 205, y = 350, w = 80, h = 40,
                    color = 0xF1BEDFFF, 
                    border_size = 4, 
                    border_color = 0xee9ab5ff,
                    border_radius = 4
                );
                text!("Cupid",
                x = 217,
                y = 364, 
                font = "TENPIXELS", 
                color = 0xcf3b7aff,
                );
            } else if self.npcName == "Client"{
                rect!(x = 205, y = 350, w = 80, h = 40,
                    color = 0xF1BEDFFF, 
                    border_size = 4, 
                    border_color = 0xee9ab5ff,
                    border_radius = 4
                );
                text!("{}", self.npcName;
                x = 217,
                y = 364, 
                font = "TENPIXELS", 
                color = 0xcf3b7aff,
                );
            }
            rect!(x = 205, y = 390, w = 625, h = 110, 
                color = 0xF1BEDFFF, 
                border_size = 4, 
                border_color = 0xee9ab5ff,
                border_radius = 4
            );
            text_box!{
                text,
                font = "TENPIXELS",
                scale = 1.22,
                color = 0xcf3b7aff,
                fixed = true,
                width = 479,
                height = 90,
                x =  230,
                y = 425,  
                             
            }
            self.assessLine(whoTalking.clone());
            
        }
    }

    pub fn assessLine(&mut self, who: String) {
        let m = pointer::world();
        if self.sLines[self.current_line_s] == "--end" {
            self.speaking = false;
            self.newSpeakSet = false;
        } else if self.sLines[self.current_line_s] == "--stare" {
            self.speakingProfile = format!("{}_stare", who);
            self.cupidTalk = true;
            self.current_line_s += 1;
        } else if self.sLines[self.current_line_s] == "--talk" {
            self.speakingProfile = format!("{}_talking", who);
            self.cupidTalk = false;
            self.current_line_s += 1;
        } else if self.sLines[self.current_line_s] == "--posterUp" { 
            self.posterUp = true;
            self.posterPos = Tween::new(-500.0)
                            .duration(150)
                            .ease(Easing::EaseInOutQuad)
                            .set(0.0);
            self.current_line_s += 1;
        } else if self.sLines[self.current_line_s] == "--posterDown" { 
            self.posterUp = false;
            self.current_line_s += 1;
        } else if self.sLines[self.current_line_s] == "--cupidTalk" {
            self.cupidTalk = true;
            self.current_line_s += 1;
        } else if self.sLines[self.current_line_s] == "--cupidNot" {
            self.cupidTalk = false;
            self.current_line_s += 1;
        } else if m.just_pressed() {
            self.current_line_s += 1; 
        } 
    }

}
