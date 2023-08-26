pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn push(&mut self, value: i32) {
        self.average = (self.average * (self.list.len() as f64) + (value as f64))
            / (self.list.len() as f64 + 1.0);
        self.list.push(value);
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.average = (self.average * (self.list.len() as f64 + 1.0) - (value as f64))
                    / (self.list.len() as f64);
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
           component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Draw Button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Draw SelectBox");
    }
}
    

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut collection = AverageCollection {
            list: vec![],
            average: 0.0,
        };
        collection.push(1);
        collection.push(2);
        collection.push(3);
        assert_eq!(collection.average(), 2.0);
        collection.remove();
        assert_eq!(collection.average(), 1.5);
    }
}
