use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    text_one: String,
    text_two: String,
    text_three: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Demo Window").show(ctx, |ui| {
            ui.label("Input One:");
            ui.text_edit_singleline(&mut self.text_one);

            ui.label("Input Two:");
            ui.text_edit_multiline(&mut self.text_two);

            ui.label("Input Three:");
            ui.text_edit_singleline(&mut self.text_three);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::accesskit::Role;
    use egui_kittest::kittest::{By, Queryable};

    #[test]
    fn single_works() {
        let mut harness =
            egui_kittest::HarnessBuilder::default().build_eframe(|_| MyApp::default());

        harness.run_steps(2);

        // Attempt to get first empty Input
        harness
            .get_by_role_and_label(egui::accesskit::Role::Window, "Demo Window")
            .query_all(By::new().value("").predicate(|node| node.is_text_input()))
            .next()
            .unwrap()
            .type_text("First");

        harness.run_steps(3);

        harness.snapshot("single_works");
    }

    #[test]
    fn one_by_one() {
        let mut harness =
            egui_kittest::HarnessBuilder::default().build_eframe(|_| MyApp::default());

        harness.run_steps(2);

        // Attempt to get first empty Input
        harness
            .get_by_role_and_label(egui::accesskit::Role::Window, "Demo Window")
            .query_all(By::new().value("").predicate(|node| node.is_text_input()))
            .next()
            .unwrap()
            .type_text("First");

        harness.run_steps(3);

        // Attempt to get first empty Input
        harness
            .get_by_role_and_label(egui::accesskit::Role::Window, "Demo Window")
            .query_all(By::new().value("").predicate(|node| node.is_text_input()))
            .next()
            .unwrap()
            .type_text("Second");

        harness.run_steps(3);

        // Attempt to get first empty Input
        harness
            .get_by_role_and_label(egui::accesskit::Role::Window, "Demo Window")
            .query_all(By::new().value("").predicate(|node| node.is_text_input()))
            .next()
            .unwrap()
            .type_text("Third");

        harness.run_steps(3);

        harness.snapshot("one_by_one");
    }

    #[test]
    fn all_at_once() {
        let mut harness =
            egui_kittest::HarnessBuilder::default().build_eframe(|_| MyApp::default());

        harness.run_steps(2);

        let mut inputs = harness
            .get_by_role_and_label(egui::accesskit::Role::Window, "Demo Window")
            .get_all_by(|node| {
                node.role() == Role::TextInput || node.role() == Role::MultilineTextInput
            });

        // Cannot run "harness.step" between inputs due to mutable borrow of harness
        inputs.next().unwrap().type_text("First");
        inputs.next().unwrap().type_text("Second");
        inputs.next().unwrap().type_text("Third");
        // End mutable borrow of harness
        drop(inputs);

        harness.run_steps(3);

        harness.snapshot("all_at_once");
    }
}
