# Modul pantry-resep — Kontrak Pawonee (pemilik) dan Pedaree (konsumen)

TownHall Pedaree: https://github.com/Coding-Skuy/Pedaree-TownHall

## Kedudukan

- **Pemilik:** Pawonee. Menetapkan skema resep dan `recipe_id`.
- **Konsumen:** Pedaree backend (pencocokan stok terhadap resep).

## Aturan konsumen (berlaku untuk backend)

1. Endpoint pencocokan: `GET /v1/kecocokan-resep?recipe_id=...`.
2. Backend mengambil definisi resep dari `PAWONEE_RECIPE_API`, lalu
   membandingkan dengan tabel `pantry_items` di basis data `pedaree`.
3. Respons berisi status per bahan: `tersedia`, `kurang`, atau `habis`.
4. Hasil pencocokan dicatat di tabel `kecocokan_resep` beserta `recipe_id`.
5. Skema resep tidak diduplikasi permanen di basis data `pedaree`.
