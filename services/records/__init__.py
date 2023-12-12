from config import connex_app, basedir, db

app = connex_app
app.add_api(basedir / "swagger.yml")


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)
