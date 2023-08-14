FROM gcr.io/distroless/static-debian11 as amd64
COPY rtrs-linux-amd64.tar.gz /app
RUN chmod +x /app

FROM gcr.io/distroless/static-debian11 as arm
ADD rtrs-linux-arm.tar.gz /app
RUN chmod +x /app

FROM ${TARGETARCH} as build
FROM gcr.io/distroless/static-debian11
COPY --from=build /app /app
ENTRYPOINT ["/app/rtrs", "--host", "0.0.0.0","--port", "80"]
