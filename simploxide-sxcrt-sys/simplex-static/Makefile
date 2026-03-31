DIST_DIR     := $(CURDIR)/dist-newstyle
STORE_DIR    := $(CURDIR)/cabal-store
GHC_VER      := $(shell ghc --numeric-version)

# Compile flags matching direct-sqlcipher.cabal (+systemlib path)
CBITS          := direct-sqlcipher/cbits
OPENSSL_CFLAGS := $(shell pkg-config --cflags openssl 2>/dev/null)
SQLCIPHER_DEFS := \
    -DSQLITE_HAS_CODEC \
    -DSQLCIPHER_CRYPTO_OPENSSL \
    -DSQLITE_TEMP_STORE=2 \
    -DHAVE_USLEEP \
    -DSQLITE_ENABLE_FTS3 \
    -DSQLITE_ENABLE_FTS3_PARENTHESIS \
    -DSQLITE_ENABLE_FTS4 \
    -DSQLITE_ENABLE_FTS5 \
    -DSQLITE_USE_URI \
    -DSQLITE_ENABLE_JSON1

.PHONY: build simplexlib sqlcipher clean veryclean verify-sqlcipher verify-simplexlib


build: simplexlib sqlcipher verify-simplexlib verify-sqlcipher


simplexlib: libsimplex.a


libsimplex.a:
	cabal --store-dir=$(STORE_DIR) build lib:simplex-chat
	bash bundle.sh


sqlcipher: libsqlcipher.a


# Build libsqlcipher.a from the pinned direct-sqlcipher submodule.
# Flags match direct-sqlcipher.cabal exactly (-systemlib +openssl path).
# Also copies sqlite3.h for SQLCIPHER_INCLUDE_DIR.
libsqlcipher.a: | direct-sqlcipher
	gcc -O2 -fPIC \
	    $(SQLCIPHER_DEFS) \
	    $(OPENSSL_CFLAGS) \
	    -I$(CBITS) \
	    -c $(CBITS)/sqlite3.c -o sqlite3.o
	ar rcs $@ sqlite3.o
	rm sqlite3.o
	cp $(CBITS)/sqlite3.h .


verify-sqlcipher: libsqlcipher.a
	@echo "=== libsqlcipher.a ==="
	@nm libsqlcipher.a | grep -q 'T sqlite3_key$$' \
	    && echo "  sqlite3_key: defined OK" \
	    || (echo "  ERROR: sqlite3_key not found"; exit 1)
	@nm libsqlcipher.a | grep -q 'T sqlite3_open$$' \
	    && echo "  sqlite3_open: defined OK" \
	    || (echo "  ERROR: sqlite3_open not found"; exit 1)


verify-simplexlib: .simplex-symbols.txt
	@echo "=== libsimplex.a ==="
	@cat .simplex-symbols.txt
	@grep -q 'T sqlite3_open' .simplex-symbols.txt \
	    && echo "  sqlite3_open: defined (SQLCipher embedded) OK" \
	    || (echo "  ERROR: sqlite3_open not defined"; exit 1)
	@grep -q 'T chat_migrate_init' .simplex-symbols.txt \
		&& echo "  chat_migrate_init: exported OK" \
	    || (echo " ERROR: chat_migrate_init is undefined in libsimplex.a"; exit 1)
	@grep -q 'T chat_send_cmd' .simplex-symbols.txt \
		&& echo "  chat_send_cmd: exported OK" \
	    || (echo " ERROR: chat_send_cmd is undefined in libsimplex.a"; exit 1)
	@grep -q 'T chat_recv_msg_wait' .simplex-symbols.txt \
		&& echo "  chat_recv_msg_wait: exported OK" \
	    || (echo " ERROR: chat_recv_msg_wait is undefined in libsimplex.a"; exit 1)
	@rm .simplex-symbols.txt


.simplex-symbols.txt: libsimplex.a
	@nm libsimplex.a | grep -E '(T sqlite3_|T chat_)' > $@


direct-sqlcipher:
	git clone https://github.com/simplex-chat/direct-sqlcipher/


clean:
	cabal clean
	rm -f libsimplex.a libsqlcipher.a sqlite3.h .simplex-symbols.txt


veryclean: clean
	rm -rf $(STORE_DIR) $(DIST_DIR) direct-sqlcipher/
