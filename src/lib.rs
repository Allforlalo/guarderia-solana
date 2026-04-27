use anchor_lang::prelude::*;

// ID del programa en Solana, se llena automáticamente al hacer "Build" en Playground
declare_id!("EF1q3VQ9VFkGT9BXY1Ydi5sieLnNxA9mN7zQDaAVeyDw");

#[program] // Macro que convierte el módulo de Rust en un programa de Solana
pub mod guarderia {
    use super::*; // Importa todos los structs y enums definidos fuera del módulo

    //////////////////////////// Instrucción: Crear Guardería /////////////////////////////////////
    /*
    Crea una PDA (Program Derived Address) vinculada al wallet del usuario.
    Esta cuenta almacenará el struct Guarderia con la lista de niños registrados.

    La PDA se deriva de:
        * Wallet address del owner
        * Program ID
        * Seed: "guarderia"

    Parámetros:
        * nombre -> nombre de la guardería -> String
    */
    pub fn crear_guarderia(context: Context<NuevaGuarderia>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let ninos: Vec<Nino> = Vec::new();

        context.accounts.guarderia.set_inner(Guarderia {
            owner: owner_id,
            nombre,
            ninos,
        });
        Ok(())
    }

    //////////////////////////// Instrucción: Registrar Niño /////////////////////////////////////
    /*
    Agrega un nuevo niño al vector de niños de la Guarderia.
    Solo el owner de la guardería puede registrar niños.

    Parámetros:
        * nombre -> nombre del niño -> String
        * edad   -> edad del niño   -> u8
    */
    pub fn registrar_nino(context: Context<NuevoNino>, nombre: String, edad: u8) -> Result<()> {
        require!(
            context.accounts.guarderia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let nino = Nino {
            nombre,
            edad,
            asistencia: true,
        };

        context.accounts.guarderia.ninos.push(nino);
        Ok(())
    }

    //////////////////////////// Instrucción: Eliminar Niño /////////////////////////////////////
    /*
    Elimina un niño del vector por su nombre.
    Error si el niño no existe o si quien llama no es el owner.

    Parámetros:
        * nombre -> nombre del niño a eliminar -> String
    */
    pub fn eliminar_nino(context: Context<NuevoNino>, nombre: String) -> Result<()> {
        require!(
            context.accounts.guarderia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let ninos = &mut context.accounts.guarderia.ninos;

        for i in 0..ninos.len() {
            if ninos[i].nombre == nombre {
                ninos.remove(i);
                msg!("Niño {} eliminado!", nombre);
                return Ok(());
            }
        }
        Err(Errores::NinoNoExiste.into())
    }

    //////////////////////////// Instrucción: Ver Niños /////////////////////////////////////
    /*
    Muestra en el log de la transacción la lista completa de niños registrados.

    Parámetros:
        Ninguno
    */
    pub fn ver_ninos(context: Context<NuevoNino>) -> Result<()> {
        require!(
            context.accounts.guarderia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Niños registrados: {:#?}", context.accounts.guarderia.ninos);
        Ok(())
    }

    //////////////////////////// Instrucción: Alternar Asistencia /////////////////////////////////////
    /*
    Cambia el estado de asistencia de un niño: true (presente) <-> false (ausente).
    Error si el niño no existe o si quien llama no es el owner.

    Parámetros:
        * nombre -> nombre del niño -> String
    */
    pub fn alternar_asistencia(context: Context<NuevoNino>, nombre: String) -> Result<()> {
        require!(
            context.accounts.guarderia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let ninos = &mut context.accounts.guarderia.ninos;
        for i in 0..ninos.len() {
            if ninos[i].nombre == nombre {
                let nuevo_estado = !ninos[i].asistencia;
                ninos[i].asistencia = nuevo_estado;
                msg!("Niño: {} asistencia: {}", nombre, nuevo_estado);
                return Ok(());
            }
        }

        Err(Errores::NinoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la guardería")]
    NoEresElOwner,
    #[msg("Error, el niño no existe en la guardería")]
    NinoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Guarderia {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    ninos: Vec<Nino>,
}

/*
Struct secundario (no es una cuenta directa en la blockchain).
Atributos necesarios:
    * AnchorSerialize/AnchorDeserialize -> para leer/escribir en la cuenta
    * Clone      -> para copiar valores
    * InitSpace  -> para calcular el espacio en memoria
    * PartialEq  -> para comparar con "=="
    * Debug      -> para mostrarlo en log con "{:#?}"
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Nino {
    #[max_len(60)]
    nombre: String,

    edad: u8,

    asistencia: bool,
}

// Contexto para crear la guardería (inicializa la PDA)
#[derive(Accounts)]
pub struct NuevaGuarderia<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Guarderia::INIT_SPACE + 8,
        seeds = [b"guarderia", owner.key().as_ref()],
        bump
    )]
    pub guarderia: Account<'info, Guarderia>,

    pub system_program: Program<'info, System>,
}

// Contexto para todas las instrucciones que modifican o leen la guardería
#[derive(Accounts)]
pub struct NuevoNino<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub guarderia: Account<'info, Guarderia>,
}
