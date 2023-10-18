use std::rc::Rc;

struct UiFactory {
  port: i32,
  instance: Option<Rc<UiFactory>>,
}

impl UiFactory {
  fn new() -> Self {
    UiFactory {
      port: 3306,
      instance: None,
    }
  }

  fn get_instance() -> Rc<UiFactory> {
    struct Helper {
      instance: Option<Rc<UiFactory>>,
    }

    impl Drop for Helper {
      fn drop(&mut self) {
        println!("UiFactory instance dropped");
      }
    }

    static mut SINGLETON: Helper = Helper { instance: None };

    let instance = unsafe {
      if SINGLETON.instance.is_none() {
        let ui_factory = Rc::new(UiFactory::new());
        SINGLETON.instance = Some(ui_factory.clone());
      }
      SINGLETON.instance.as_ref().unwrap().clone()
    };

    instance
  }

  fn get_data(&self) -> i32 {
    self.port
  }
}

// fn main() {
//   let instance1 = UiFactory::get_instance();
//   let instance2 = UiFactory::get_instance();
//
//   println!("UiFactory data: {}", instance1.get_data());
// }
