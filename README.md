# Guardería Solana

Programa desarrollado en Solana usando Rust y el framework Anchor.

## Descripción

Sistema de gestión de una guardería desplegado en la blockchain de Solana.
Permite registrar niños, eliminarlos, ver la lista y controlar su asistencia diaria.
Cada guardería está vinculada a un wallet mediante una PDA (Program Derived Address).

## Instrucciones del programa

- `crear_guarderia` - Crea una nueva guardería vinculada al wallet del owner
- `registrar_nino` - Registra un niño con nombre, edad y asistencia activa por defecto
- `eliminar_nino` - Elimina un niño por nombre
- `ver_ninos` - Muestra la lista completa de niños en el log de la transacción
- `alternar_asistencia` - Cambia el estado de asistencia de un niño (presente/ausente)

## Tecnologías

- Rust
- Anchor Framework
- Solana Devnet

## Conceptos aplicados

- CRUD completo
- PDA (Program Derived Address) por wallet
- Validación de owner en cada instrucción
- Manejo de errores personalizados
