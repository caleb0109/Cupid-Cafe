mod button;
mod reader;

use crate::{button::button::Button};

use turbo::{camera::y, *};

#[turbo::game]
struct GameState {
    // Add fields here   
    pub reader: reader::Reader, 
    uiButtons: [Button; 7],

    pub day: i32,
    pub dayMax: i32,
    pub interact: i32,
    pub progressMax: i32,
    pub talking: String,
    pub whoTalking: String,
    pub cameraPos: (i32,i32),
    pub posterTween: Tween<f32>,
    
    pub npc: Vec<Button>,
    pub npcProgress: Vec<i32>,
    pub npcDescription: Vec<String>,
    pub currClient: i32,
    pub answer: String,
    pub answerDesc: Vec<String>,
    pub bingleCheck: bool,

    //char Description
    pub artistDesc: [String; 5],
    pub performDesc: [String; 5],
    pub bingleDesc: [String; 5],
    pub baristaDesc: [String; 5],
    
    pub intro: bool,
    pub clientEnd: bool,
    pub choosing: bool,
    pub selected: (bool,usize),
    pub ending: i32,
    pub noteAct: bool,
    pub notePage: i32,

}
impl GameState {
    pub fn new() -> Self {
        // initialize your game state
        Self { 
            //file reader, UIButtons, NPCs, 
            //and NPC progress (how many times npc was interacted with)
            reader: reader::Reader::new(),
            uiButtons: [
                Button::new("nextDay", (359.5, 300.0, 170.0, 60.0), false),
                Button::new("notepad", (800.0,110.0,75.0,85.0), false),
                Button::new("start", (1199.0, 380.0, 260.0, 70.0), false),
                Button::new("yes", (230.0, 200.0, 130.0, 100.0), false),
                Button::new("no", (560.0, 200.0, 130.0, 100.0), false),
                Button::new("left", (129.0,230.0,25.0,40.0), false),
                Button::new("right", (746.0,230.0,25.0,40.0), false),
            ],

            
            //general 
            day: 0,
            dayMax: 4,
            interact: 3,
            progressMax: 4,
            talking: "".to_string(),
            whoTalking: "".to_string(),
            cameraPos: (1333,250),
            posterTween: Tween::new(-500.0)
                            .duration(200)
                            .ease(Easing::EaseInOutQuad)
                            .set(0.0),

            //important
            npc: Vec::new(),
            npcProgress: Vec::new(),
            npcDescription: Vec::new(),
            currClient: 0,
            answer: "".to_string(),
            answerDesc: vec!["".to_string()],
            bingleCheck: false,

            //description
            artistDesc: [
                "".to_string(),
                "Furry artist".to_string(),
                "ambivert".to_string(),
                "dancer".to_string(),
                "colorful hair".to_string(),
            ],
            performDesc: [
                "".to_string(),
                "5'7\" (actually 5'\"6)".to_string(),
                "extorverted".to_string(),
                "collecting feminist literature".to_string(),
                "ceremonial grade matcha".to_string(),
            ],
            bingleDesc: [
                "".to_string(),
                "handsome".to_string(),
                "meows cutely".to_string(),
                "big foodie".to_string(),
                "10/10 loafs".to_string(),
            ],
            baristaDesc: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],


            //gamestates
            intro: false,
            clientEnd: false,
            choosing: false,
            selected: (false, 0),
            ending: 0,
            noteAct: false,
            notePage: 0,
            
        }
    }
    pub fn update(&mut self) {
        // This is where your main game loop code goes
        // The stuff in this block will run ~60x per sec

        //gets mouse position
        let mut select: (f32,f32) = (0.0,0.0);
        let m = pointer::world();
        let(mx, my) = m.xy();
        let x = mx as f32;
        let y = my as f32;

        //camera set
        camera::set_xy(self.cameraPos.0, self.cameraPos.1);
        
        

        sprite!("titleScreen", x = 890, y = 0);
        if self.intro && self.reader.speaking {
            if self.reader.posterUp {
                let val = self.reader.posterPos.get();
                sprite!("poster", x = 0, y = val);
            }
            self.reader.drawText(&"intro".to_string(), &"intro".to_string());
        } else if self.intro && !self.reader.speaking {
            self.day += 1;
            self.currClient += 1;
            self.clientUpdate(self.currClient);
            self.intro = false;
        }
        
        if !self.intro && self.day <= self.dayMax && !self.clientEnd{
            sprite!("cafe", x = 0, y = 0);
        } else if self.clientEnd {
            sprite!("cafe", x = 0, y = 0);
            sprite!("talkBG", x = 0, y = 0);
        }
        
        if self.clientEnd && !self.reader.speaking && !self.choosing{
            self.choosing = true;
        }



        if self.ending == 2 && !self.reader.speaking {
            *self = GameState::new();
            text!("bruz", x = 40, y = 100, font = "TENPIXELS");
        } else if self.ending == 1 && !self.reader.speaking {
            self.npc = Vec::new();
            self.answerDesc = Vec::new();
            self.currClient += 1;
            self.clientUpdate(self.currClient);
            self.ending = 0;
            self.day = 0;
            self.talking = self.currClient.to_string() + &"client".to_string() + &1.to_string();
            self.reader.npcName = "Client".to_string();
            self.clientEnd = false;
            self.choosing = false;
            self.selected.0 = false;
            self.selected.1 = 0;
            self.noteAct = false;
            self.reader.speaking = true;
        }


        let mut xOffset: f32 = 50.0;
        //for loop to check npcs during the day
        for n in 0..self.npc.len() {

            if self.noteAct {
                text!("{}",self.noteAct; x = 0, y = 10);
                self.npc[n].action = false;
            } else if self.selected.0 {
                self.npc[n].action = false;
            } else {
                let select = self.npc[n].check(select);
            }
            
            if self.interact <= 0 || self.day == 0{
                self.npc[n].action = false;
            }
            if self.reader.speaking {
                self.npc[n].action = false;
            }
            if self.clientEnd && !self.choosing{
                self.npc[n].action = false;
            }
            
            if self.npc[n].action {
                match self.npc[n].text.as_str() {
                    "performative" => {
                        if self.choosing {
                            self.selected.0 = true;
                            self.selected.1 = n;
                        } else {
                            self.npcInteract(n);
                        }
                        
                    }
                    "artist" => {
                        if self.choosing {
                            self.selected.0 = true;
                            self.selected.1 = n;
                        } else {
                            self.npcInteract(n);
                        }
                    }
                    "barista" => {
                        if self.choosing {
                            self.selected.0 = true;
                            self.selected.1 = n;
                        } else {
                            self.npcInteract(n);
                        }
                    }
                    "bingleFart" => {
                        if self.choosing {
                            self.selected.0 = true;
                            self.selected.1 = n;
                        } else {
                            self.npcInteract(n);
                        }
                    }
                    _=> {}
                }
            }
            
            if self.day != 0 && !self.clientEnd{
                //self.npc[n].tempDraw(&self.npc[n].text);
                self.npc[n].draw(false);
            }

            if self.choosing {
                // self.npc[n].hitbox.0 = 220.0 + xOffset;
                // self.npc[n].hitbox.1 = 200.0;
                // xOffset += 100.0;

                // self.npc[n].tempDraw(&self.npc[n].text);
                // text!("{}", self.npc[n].text; 
                //     x = self.npc[n].hitbox.0 + (self.npc[n].hitbox.2/4.0),
                //     y = self.npc[n].hitbox.1 + (self.npc[n].hitbox.3/4.0),
                //     color = 0x22406eff,
                //     font = "TENPIXELS");
                //self.npc[n].tempDraw(&self.npc[n].text);
                self.npc[n].draw(false);
            }
            
        }

        if self.noteAct {
            sprite!("talkBG", x = 0, y = 0);
        }
                if self.clientEnd && self.choosing {
            rect!(x = 205, y = 390, w = 625, h = 110,
                color = 0xF1BEDFFF, 
                border_size = 4, 
                border_color = 0xee9ab5ff,
                border_radius = 4
            );
            text_box!{
                "Who will you choose?",
                font = "TENPIXELS",
                scale = 1.22,
                color = 0xfae3deff,
                fixed = true,
                width = 479,
                height = 90,
                x =  230,
                y = 425,    
                             
            }
        }
        if self.selected.0 {
            sprite!("talkBG", x = 0, y = 0);
            let insert = format!("{}_stare", self.npc[self.selected.1].text);
            if self.npc[self.selected.1].text == "bingleFart" {
                sprite!(&insert.to_string(), x = 325, y = 165);
            } else {
                sprite!(&insert.to_string(), x = 338, y = 10);
            }
            
            rect!(x = 205, y = 390, w = 625, h = 110, 
                color = 0xF1BEDFFF, 
                border_size = 4, 
                border_color = 0xee9ab5ff,
                border_radius = 4
            );
            text_box!{
                "Is this the one: {}",
                self.npc[self.selected.1].text;
                font = "TENPIXELS",
                scale = 1.22,
                color = 0xfae3deff,
                fixed = true,
                width = 479,
                height = 90,
                x =  230,
                y = 425,    
                             
            }
        }
        
                //for loop to check UI Buttons
        for n in 0..self.uiButtons.len() {
            let select = self.uiButtons[n].check(select);
            
            if self.reader.speaking || self.intro{
                self.uiButtons[n].action = false;
            }
            if self.interact >= 1 && n == 0 {
                self.uiButtons[n].action = false;   
            }
            if self.selected.0 && n <= 2 {
                self.uiButtons[n].action = false;
            } 
            if !self.selected.0 && n >= 3 && n <= 4{
                self.uiButtons[n].action =false;
            }
            if self.choosing && n == 0 && n >= 2 && !self.noteAct {
                self.uiButtons[n].action = false;
            }
            if self.selected.0 && n == 3 && !self.bingleCheck{
                self.uiButtons[n].action = false;
            }
            if self.uiButtons[n].action {
                match n {
                    //nextday
                    0 => {
                        //client update when its past first client,
                        //make sure to move this to proper button
                        if self.day == 0 {
                            self.currClient += 1;
                            self.clientUpdate(self.currClient);
                        }
                        if self.day >= self.dayMax {
                            self.talking = self.currClient.to_string() + &"clientEnd".to_string() + &1.to_string();
                            self.reader.speaking = true;
                            self.reader.speakingProfile = "".to_string();
                            self.clientEnd = true;
                        } else {
                            self.day += 1;
                        }
                        
                        self.interact = 3;
                        self.uiButtons[n].action = false;
                    }
                    //notepad
                    1 => {
                        if self.noteAct {
                            self.noteAct = false
                        } else {
                            self.noteAct = true;
                            self.notePage = 1;
                        }
                        self.uiButtons[n].action = false;
                    }
                    //start button
                    2 => {
                        self.intro = true;
                        self.reader.speaking = true;
                        self.cameraPos.0 = 444;
                        self.uiButtons[n].action = false;
                    }
                    //yes in end
                    3 => {
                        if self.npc[self.selected.1].text == self.answer {
                            self.talking = self.currClient.to_string() + &"clientEnd".to_string() + &2.to_string() + &"Good".to_string();
                            self.reader.speaking = true;
                            self.ending = 1;
                            self.reader.drawText(&self.talking, &self.whoTalking);
                        } else {
                            self.talking = self.currClient.to_string() + &"clientEnd".to_string() + &2.to_string() + &"Bad".to_string();
                            self.reader.speaking = true;
                            self.ending = 2;
                            self.reader.drawText(&self.talking, &self.whoTalking);
                        }
                        self.bingleCheck = false;
                        
                        self.uiButtons[n].action = false;
                    }
                    //no in edn
                    4 => {
                        self.selected.0 = false;
                        self.selected.1 = 0;
                        self.bingleCheck = false;
                        self.uiButtons[n].action = false;
                    }
                    //left notepad
                    5 => {
                        self.notePage = 1;
                        self.uiButtons[n].action = false;
                    }
                    //right notepad
                    6 => {
                        self.notePage = 2;
                        
                        self.uiButtons[n].action = false;
                    }
                    _=> {}
                }
            }
            
            //prints temp button
            if self.interact == 0 && n == 0 {
                sprite!("talkBG", x = 0, y = 0);
                self.uiButtons[0].draw(false);
            }
            if !self.intro && n != 0 && n <= 2{
                self.uiButtons[n].draw(false);
            }
            if self.selected.0 && n >= 3 && n <= 4{
                self.uiButtons[n].draw(false);
            }
        }

        if self.day > 0 || self.currClient >= 1{
            //temp prints
            text!("DAY: {}", self.day; x = 790, y = 80, font = "TENPIXELS", color = 0x000000ff);
            match self.interact {
                3 => { sprite!("health3", x = 30, y = -23)}
                2 => { sprite!("health2", x = 30, y = -23)}
                1 => { sprite!("health1", x = 30, y = -23)}
                0 => { sprite!("health0", x = 30, y = -23)}
                _=> {}
            }
            match self.day {
                4 => {sprite!("day4", x = 0, y = 0)}
                3 => {sprite!("day3", x = 0, y = 0)}
                2 => {sprite!("day2", x = 0, y = 0)}
                1 => {sprite!("day1", x = 0, y = 0)}
                _=> {}
            }
            
            if self.reader.speaking {
                if !self.clientEnd {
                    sprite!("talkBG", x = 0, y = 0);
                }
                self.reader.drawText(&self.talking, &self.whoTalking);
            } 
            if self.noteAct {
                sprite!("notepadOpen", x = 0, y = 0);
                let mut yoffSet = 20;
                if self.notePage == 1 {
                    if self.currClient == 1 {
                        sprite!("notepad1", x = 0, y = 0);
                        for n in 0..self.npcProgress.len() {
                            //text!("{}", self.npcProgress[n]; x = 10, y = 10);
                            for m in 0..self.npcProgress[n] + 1 {
                                text!("{}", m; x = 10, y = 10);
                                match n {
                                    3 => {
                                        if self.npcProgress[n] == 0 {
                                            text!("?????", x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                        } else {
                                            text!("{}", self.baristaDesc[ m as usize]; x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                            
                                        }
                                    }
                                    2 => {
                                        if self.npcProgress[n] == 0 {
                                            text!("?????", x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                        } else {
                                            text!("{}", self.bingleDesc[ m as usize]; x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                            
                                        }
                                    }
                                    1 => {
                                        if self.npcProgress[n] == 0 {
                                            text!("?????", x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                        } else {
                                            text!("{}", self.artistDesc[ m as usize]; x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                            
                                        }
                                    }
                                    0 => {
                                        if self.npcProgress[n] == 0 {
                                            text!("?????", x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                        } else {
                                            text!("{}", self.performDesc[ m as usize]; x = 620, y = 110 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                                            
                                        }
                                    }
                                    _=> {}
                                }
                                yoffSet += 20;
                            }
                        }
                    } else if self.currClient == 2 {
                        sprite!("notepad2", x = 0, y = 0);
                    }
                    self.uiButtons[6].draw(false);
                } else if self.notePage == 2 {
                    text!("They supposedly want: ", x = 220, y = 120, font = "TENPIXELS", color = 0x000000ff);
                    let mut yoffSet = 10;
                    for n in 0..self.answerDesc.len() {
                        text!("{}", self.answerDesc[n]; x = 220, y = 120 + yoffSet, font = "TENPIXELS", color = 0x000000ff);
                        yoffSet += 40;
                    }
                    self.uiButtons[5].draw(false);
                }
            }
            if self.currClient == 2 && !self.reader.speaking && self.day == 0{
                self.day += 1;
            }
            if self.selected.0 && !self.bingleCheck && m.just_released() {
                self.bingleCheck = true;
            }
            text!("{}", self.npcProgress[1]; x = 20, y =20);
        }
        
    }

    pub fn npcInteract (&mut self, n: usize) {
        self.interact -= 1;
        if self.npcProgress[n] < self.progressMax {
            self.npcProgress[n] += 1;
        }
        self.reader.speaking = true;
        self.whoTalking = self.npc[n].text.clone();
        self.talking = self.currClient.to_string() + &self.npc[n].text.to_string() + &self.npcProgress[n].to_string();
        self.npc[n].action = false;
    }

    pub fn clientUpdate (&mut self, client: i32) {
        match client {
            1 => {
                self.npc.push(Button::new("performative", (690.0, 220.0, 110.0, 180.0), false));
                self.npc.push(Button::new("artist",(50.0, 174.0, 90.0, 225.0), false));
                self.npc.push(Button::new("bingleFart",(290.0, 225.0, 50.0, 50.0), false));
                self.npcProgress = vec![0,0,0];
                self.answer = "artist".to_string();
                self.answerDesc.push("Tall".to_string());
                self.answerDesc.push("Artsy".to_string());
            }
            2 => {
                self.npc.push(Button::new("performative", (690.0, 220.0, 110.0, 180.0), false));
                self.npc.push(Button::new("artist",(50.0, 174.0, 90.0, 225.0), false));
                self.npc.push(Button::new("bingleFart",(290.0, 225.0, 50.0, 50.0), false));
                self.npc.push(Button::new("barista", (334.0,135.0, 100.0,130.0), false));
                self.npcProgress = vec![0,0,0,0];
                self.answer = "barista".to_string();
                self.answerDesc.push("Listens well".to_string());
                self.answerDesc.push("Not Fake".to_string());
                self.answerDesc.push("Skilled".to_string());
            }
            _=> {}
        }

    }
}
