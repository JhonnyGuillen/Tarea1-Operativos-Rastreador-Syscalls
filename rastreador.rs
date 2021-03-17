use std::env;

fn imprimirSystemCalls(prog: String, opProg: String){
    println!("systemcalls de {} con {}", prog, opProg);

}

fn imprimirSystemCallsPausa(prog: String, opProg: String){
    println!("systemcalls con pausa de {} con {}", prog, opProg);
}

fn ayuda(){
    println!("Uso del programa: El programa retornara los system calls que se usan en el programa que se pasa");
    println!("Comando: ./rastreador [opciones de rastreador] 'prog' [opciones de 'prog']");
    println!("Opcion de rastreador puede ser '-v' para imprimir un mensaje cada que encuentra un system calls, o puede ser '-V' para imprimir un mensaje cada que encuentra un system call, pero entre cada mensaje hay una pausa hasta presionar una tecla del teclado");
    println!("Opcion de 'prog' es opcional, depende si prog requiere argumentos o no");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        //Sin argumentos
        3 => {
            let opRastredor: &str = &args[1];
            let prog: &str = &args[2];
            let opProg: &str = "";
            match &opRastredor[..] {
                "-v" => imprimirSystemCalls(prog.to_string(), opProg.to_string()),
                "-V" => imprimirSystemCallsPausa(prog.to_string(), opProg.to_string()),
                _ => {
                    eprintln!("Error: Opcion de Rastreador invalido");
                }
            }
        }
        4 => {
            let opRastredor: &str = &args[1];
            let prog: &str = &args[2];
            let opProg: &str = &args[3];
            match &opRastredor[..] {
                "-v" => imprimirSystemCalls(prog.to_string(), opProg.to_string()),
                "-V" => imprimirSystemCallsPausa(prog.to_string(), opProg.to_string()),
                _ => {
                    eprintln!("Error: Opcion de Rastreador invalido");
                }
            }
        }
        _ =>{
            eprintln!("Error: Argumentos invalidos");
            ayuda();
        }
    }
}
