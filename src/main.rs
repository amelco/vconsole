use vconsole::VConsole;

mod vconsole;

fn main() {
    
    // Cria GUI criando uma instancia de VConsole com "cartucho" (código assembly) carregado.

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(VConsole::default())),
    );

    // Descobrir como chamar a função execute da CPU de VConsole em um loop e ir mostrando os resultados

}
