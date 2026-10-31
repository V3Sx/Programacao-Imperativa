use std::alloc::{Layout, alloc};
use std::ptr;

fn main(){
    // exercicio_1();
    // exercicio_2();
}

fn exercicio_2(){
    let layout = Layout::array::<i32>(3).unwrap(); //alocando espaco para tres inteiros
    let ptr = unsafe {alloc(layout)};

    //da pra definir a funcao como unsafe tambem
    
    unsafe{
        let ptr = ptr as *mut u8;
        ptr.add(0).write(10);
        ptr.add(1).write(20);
        ptr.add(2).write(30);

        println!("os valores sao: {}, {} e {}", ptr.add(0).read(), ptr.add(1).read(), ptr.add(2).read());

        dealloc(ptr, layout);
    }
}

fn exercicio_1() {
    let layout  = Layout::new::<u32>(); //definindo layout para um unico u32

    //vou alocar a memoria usando o layout
    let ptr = unsafe { alloc(layout)};

    unsafe {
        ptr::write(ptr as *mut u32, 42); //armazenando o valor 42

        //leitura
        let valor = ptr::read(ptr as *const u32);
        println!("Valor armazenado: {}", valor);

        //liberacao da memoria bruta e ponto crucial da aula
        dealloc(ptr, layout);
    }
}

use std::alloc::dealloc;