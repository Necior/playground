FROM python:3.10

WORKDIR /usr/src/app
RUN pip install --no-cache-dir flask gunicorn pymongo
COPY app.py .

ENTRYPOINT ["gunicorn", "--bind", "0.0.0.0:8000", "app:app"]

