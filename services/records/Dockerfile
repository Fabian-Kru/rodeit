FROM python:3.12-slim

ARG DEBIAN_FRONTEND=noninteractive

WORKDIR /app
COPY . .

RUN pip install --upgrade pip && \
    pip install -r requirements.txt

EXPOSE 8000

ENTRYPOINT ["python3"]
CMD ["__init__.py"]
