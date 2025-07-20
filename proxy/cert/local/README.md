# Self-signed SSL certificate for local development

_The private key is not a secret. Do not use it for anything valuable._

* Private key: `domain.key`
* Certificate signing request: `domain.csr`
* Temporary certificate: `domain.crt`

Inspect the signed certificate with OpenSSL:

    openssl x509 -text -noout -in domain.crt

