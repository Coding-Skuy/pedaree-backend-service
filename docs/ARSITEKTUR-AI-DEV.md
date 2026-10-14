> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# ARSITEKTUR-AI-DEV — Stack terkunci dan batas context (pedaree-backend-service)

Mengacu: Pedaree-TownHall `platform/versions/v1.0.0` (https://github.com/Coding-Skuy/Pedaree-TownHall/tree/main/platform/versions/v1.0.0), beku 2026-10-07. Dokumen ini MELENGKAPI `versions/v1.0.0/docs/40-TECHSTACK.md` dan `versions/v1.0.0/docs/60-BLAST-RADIUS.md` (keduanya beku v1.0.0 dan tidak diedit; isinya hanya ringkasan satu paragraf). Tidak menduplikasi BRD/PRD/FRD/FSD/SRS.

Baris Pedaree townhall-roadmap §2: point of concern "FEFO; stockout <8%; waste <1.500 g/Rp50.000/bln"; batas context "13 satuan baku; konsumen modul `pantry-resep` milik Pawonee". Rujukan lintas divisi: KONTEKS §3.1, §6, §8, §9.

## 1. Matriks stack

| Komponen | Kondisi saat ini (tercatat dari manifes/README) | Target terkunci v1.0.0 |
|---|---|---|
| Rust | 1.84.1 (`rust-version`) | stabil terbaru, edisi 2021 (pin eksak saat kickoff) |
| Web framework | axum 0.7.9 | axum 0.8.x (path `{id}`) |
| Async | tokio 1.42.0 | tokio 1.x |
| DB | sqlx 0.8.3, Postgres 16.6 | sqlx 0.8.x + `migrate!`, Postgres 16.x, DB `pedaree` |
| Middleware | belum ada | tower-http (Trace + Timeout) |
| Observabilitas | tracing 0.1.41 | tracing |
| ID | uuid v4 | uuid v7 |
| Auth | belum ada | jsonwebtoken + argon2 |
| Uji | axum-test 16.0.0 | `#[sqlx::test]` + uji kontrak |
| Skema stok | `pantry_items` datar + `mutasi_stok` masuk/keluar | `pantry_items` + `stok_batch` + `mutasi_stok` (5 jenis) |
| Event keluar | topik Kafka `pedaree.stok.v1` (README) | outbox -> `POST /sinyal/pedaree-stok` TitipO (lihat `pedaree-data-pipeline`) |

## 2. Drift yang tercatat

- Axum 0.7.9 dan rute tanpa sintaks `{id}`; target Axum 0.8.
- `GET /kesehatan` hanya endpoint; belum ada auth, `AppState`, `AppError`, serta rute `/pantry/*` (PDR-BE-002..005).
- Skema `001_init.sql` tidak mengenal batch/FEFO, lokasi simpan, kategori, dan mutasi `PAKAI/KOREKSI/PINDAH/BUANG`.
- README menyebut Redis 7.4.1 untuk pengingat kedaluwarsa; keputusan terkunci: job server `expiry-sweep` 06:00 WIB (BRD 20 BR-202), tidak butuh Redis kecuali diputuskan ADR baru.

## 3. Boleh disentuh repo ini

- Seluruh kode `src/`, `migrations/`, dan `docs/api/` repo ini.
- DB logis `pedaree` (tabel stok, batch, mutasi, outbox sinyal).
- Memanggil TitipO `POST /sinyal/pedaree-stok` dengan service key.

## 4. Dilarang

- Query langsung ke DB TitipO, Pawonee, Lumbung, atau divisi lain.
- Mengubah modul `:shared:pantry-resep` atau menyalin skema resep Pawonee ke DB `pedaree` (hanya `recipe_id` rujukan).
- Membuat pesanan TitipO otomatis dari sinyal.
- Menyentuh DB divisi lain (DB per service; Titeny hanya read-only ke DB sumber).
- Membuat pesanan TitipO otomatis dari sinyal stok (hanya anjuran, relevansi 48 jam).
- Mengubah modul `:shared:pantry-resep` (milik Pawonee), daftar 13 satuan, ambang kedaluwarsa, atau audiens JWT tanpa PR TownHall + ADR.

## 5. Kontrak yang diproduksi

- API v1 `https://api.pedaree.chefgenie.id/v1` dan OpenAPI 3.1 `docs/api/openapi.yaml`.
- Penerbitan JWT audiens `pedaree` + refresh.
- Sinyal `pedaree.stok.v1` (stok.menipis, stok.habis, kedaluwarsa.dekat) ke TitipO `POST /sinyal/pedaree-stok`, header `X-Service-Key`.

## 6. Kontrak yang dikonsumsi

- API/artefak Pawonee untuk `recipe_id` dan modul `:shared:pantry-resep` (hanya baca/pakai).
- Rahasia (`DATABASE_URL`, kunci JWT, service key) dari `pedaree-infra-devops`.

## 7. Dampak bila berubah

- Perubahan skema DB butuh migrasi baru + migrasi balik; memengaruhi klien KMP/web dan pipeline.
- Perubahan payload sinyal memecahkan penerima TitipO; bump versi + dual-read 30 hari.
- Perubahan audiens JWT memutus autentikasi semua klien.

## Batasan

- Dokumen ini hanya mencatat stack dan batas; tidak mengubah manifes atau kode. Kondisi saat ini dicatat apa adanya.
- Versi dikunci eksak di manifes saat kickoff coding; perubahan dicatat lewat ADR baru.
- JWT audiens `pedaree`, DB logis `pedaree`, dan kepemilikan Pawonee atas `:shared:pantry-resep` tidak berubah oleh dokumen ini.
