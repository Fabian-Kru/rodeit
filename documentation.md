# RodeIt
- 🎢 Record what coasters you have ridden

- 📋 Create a bucket list with coasters you want to ride

- 🙋 See records and bucket lists of other users

- 🏆 Visualize statistics, like most ridden and most wanted coasters
# Dokumentation
- [OpenAPI-Schnitstellenbeschreibung](https://emonadeo.github.io/rodeit/)
- [Webseite](https://rodeit.mabezi.de/)

# Angaben zum Team

## Vorgehensweise
**Wöchentliche Meetings**
> - Klärung von Fragen/Problemen
> - Besprechen des weiteren Vorgehens
> - Coden

**Anfängliche Planung**

> 1. Ideensuche (Hobby -> Webseite)
> 2. Grobe Pläne, andere Webseiten angeschaut
> 3. Brainstorming
> 4. Absprache der eingesetzen Software & Plattformen
> 5. Implementierung


### Brainstorming

![](https://md.ascii.coffee/uploads/d9de2f9d-7ec9-4700-a83d-629f54a8d780.svg)


## Autoren
- Emanuel Pilz
- Fabian Krusch
- Markus Ziehe



# Verwendete Plattform / Software

## Hinweise für Installation
**Docker muss bereits installiert sein!**

`Linux, Mac:`
1. Im Hauptverzeichnis **"./setup.sh"** ausführen
2. **"docker compose up"** ausführen

`Windows:`
1. Alle Docker Images bauen
2. **"docker compose up"** ausführen

## Plattformen
- GitLab
-- GitLab Registry (Speichern der Docker-Images)
- GitHub-Pages (Hosten von Scalar)
- Scalar (OpenAPI-Schnittstellenbeschreibung)
- Docker

## Services

### `bucket_list`
- **Rust**
- [axum](https://github.com/tokio-rs/axum) (Web-Framework)
- [aide](https://github.com/tamasfe/aide) (code-first OpenAPI)
- [SurrealDB](https://surrealdb.com/)
- [anyhow](https://github.com/dtolnay/anyhow)
- [auth](#auth)

### `records`
- **Python**
- SQLite

#### Libraries
- [Flask](https://flask.palletsprojects.com/en/3.0.x/)
- [Connexion](https://connexion.readthedocs.io/en/latest/)
- [Marshmallow](https://marshmallow.readthedocs.io/en/stable/)
- [SQLAlchemy](https://www.sqlalchemy.org/)
- [Python JOSE](https://python-jose.readthedocs.io/en/latest/)

### `user`
- **Rust**
- SQLite
- [Axum](https://github.com/tokio-rs/axum) (Web-Framework)
- [sqlx](https://github.com/launchbadge/sqlx)
- [tokio](https://tokio.rs/)
- [utoipa](https://github.com/juhaku/utoipa)
- [serde](https://serde.rs/)
- [auth](#auth)

## Teilpakete (packages)

### `auth`

> Wird von `bucket_list` und `user` genutzt

- [jsonwebtoken](https://jwt.io/)

# Anleitung


## `/signup`


## `/login` 
![](https://md.ascii.coffee/uploads/87d4d68e-5a2c-4f52-a702-90c6cca090b2.png)

## `/logout` 
Loggt den Nutzer aus und leitet auf `/` weiter.

## `/coaster`
![](https://md.ascii.coffee/uploads/23908e18-40ec-4609-826f-3c0473596f69.png)


## `/coaster/{id}`

## `/users`

## `/users/{id}`


## Feedback

### positives
- hat spaß gemacht
- Teamarbeit


### negatives/ Kritik / Verbesserungsvorschläge
- VM hatte zu wenig Leistung um Rust-Programme zu compilen => Dadurch sind auch keine Pipelines nutzbar gewesen
