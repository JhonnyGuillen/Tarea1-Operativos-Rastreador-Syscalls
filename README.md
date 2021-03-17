# Tarea1-Operativos-Rastreador-Syscalls
En esta tarea consiste en realizar un rastreador de System calls. Los System Calls, o sys-
calls, son llamadas a funciones internas de las computadoras que trabajan a bajo nivel.
El objetivo de esta tarea es imprimir un mensaje con una descripción de los syscalls que
son usados en un procedimiento que se llame junto al rastreador de syscalls.

# Pre-requisitos
Para la presente tarea es necesario contar con Rust instalado, por lo que es un requisito
correr los siguientes comandos en una terminal de Ubuntu.
curl -- proto '= https ' -- tlsv 1.2 - sSf https :// sh . rustup . rs | sh

# Compilación
rustc rastreador . rs

# Ejecución
./ rastreador [ - v / - V ] ' nombre del programa ' [ argumentos del programa ]
