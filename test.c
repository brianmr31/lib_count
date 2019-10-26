#include "test.h"

int main (void) {
    rust_count("SELECT count(tokn_id) from tokns limit 1");
}
