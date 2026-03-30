STORE_DIR    := $(CURDIR)/cabal-store
GHC_VER      := $(shell ghc --numeric-version)

# SQLCipher linking: static (bundled amalgamation, default) or dynamic (system libsqlcipher)
SQLCIPHER_LINKAGE ?= dynamic

ifneq ($(SQLCIPHER_LINKAGE),static)
ifneq ($(SQLCIPHER_LINKAGE),dynamic)
$(error SQLCIPHER_LINKAGE must be 'static' or 'dynamic', got '$(SQLCIPHER_LINKAGE)')
endif
endif

PREV_LINKAGE := $(shell cat .sqlcipher_linkage 2>/dev/null)

.PHONY: build bundle clean veryclean cabal.project.local

build: cabal.project.local
ifneq ($(PREV_LINKAGE),$(SQLCIPHER_LINKAGE))
	@echo "SQLCipher linkage changed ($(PREV_LINKAGE) -> $(SQLCIPHER_LINKAGE)), cleaning store..."
	cabal --store-dir=$(STORE_DIR) clean
	rm -rf $(STORE_DIR)
endif
	cabal --store-dir=$(STORE_DIR) build lib:simplex-chat
	@echo $(SQLCIPHER_LINKAGE) > .sqlcipher_linkage

bundle: build
	SQLCIPHER_LINKAGE=$(SQLCIPHER_LINKAGE) bash bundle.sh

cabal.project.local:
ifeq ($(SQLCIPHER_LINKAGE),dynamic)
	@echo "SQLCipher linkage: dynamic (system libsqlcipher)"
	@printf 'ignore-project: False\n\npackage direct-sqlcipher\n  flags: +systemlib +openssl\n  extra-include-dirs: /usr/include/sqlcipher\n' > $@
else
	@echo "SQLCipher linkage: static (bundled amalgamation + OpenSSL)"
	@printf 'ignore-project: False\n\npackage direct-sqlcipher\n  flags: -systemlib +openssl\n' > $@
endif

clean:
	cabal clean
	rm -f libsimplex.a cabal.project.local .sqlcipher_linkage

veryclean: clean
	cabal --store-dir=$(STORE_DIR) clean
	rm -rf $(STORE_DIR)
