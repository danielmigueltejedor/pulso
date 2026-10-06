# Pulso

Composición y motion graphics de Nodalix.

El motor de este repositorio es el código de [storytold/effectcraft](https://github.com/storytold/effectcraft) en `2fadd14f09bb064cab8f1f8fe85af601a08b6e53`.
La licencia de ese código se conserva, igual que `NOTICE` y `docs/upstream-readme.md`.
Pulso es el nombre del producto. El motor puede cambiar sin esperar a upstream.
Un sync posterior compara commits. No se sustituye este árbol con un reset.

## Compilar

```bash
cargo run -p effectcraft
```

El binario de escritorio se llama `pulso`.

## Estado

La shell que entra en este import es la de upstream. La interfaz GTK 4 / libadwaita de Nodalix no está cableada todavía. El motor sí está en este repositorio, con sus tests.
