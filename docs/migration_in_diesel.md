# Migraciones con Diesel en Rust 🦀

Para realizar migraciones con **Diesel** en Rust necesitamos PostgreSQL. En este proyecto usamos PostgreSQL mediante Docker.

## 1. Configurar la base de datos

```bash
docker compose up -d
```

En `.env`:

```env
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/tutorial"
```

## 2. Inicializar Diesel

```bash
diesel setup
```

Esto crea la carpeta `migrations/`.

## 3. Crear una migración

```bash
diesel migration generate create_database
```

Esto crea:

```text
migrations/
└── create_database/
    ├── up.sql
    └── down.sql
```

- `up.sql` → crea o modifica tablas.
- `down.sql` → revierte esos cambios.

## 4. Ejecutar la migración

```bash
diesel migration run
```

Diesel aplica los cambios en PostgreSQL y genera:

```text
src/schema.rs
```

Este archivo representa el esquema de la base de datos para utilizarlo desde Rust.

## Comandos principales

```bash
diesel setup
diesel migration generate nombre_migracion
diesel migration run
diesel migration revert
diesel migration list
```