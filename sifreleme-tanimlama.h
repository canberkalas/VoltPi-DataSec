#ifndef ENCRYPTION_H
#define ENCRYPTION_H

#include <stdint.h>
#include <stddef.h>

void aes128_cbc_encrypt(const uint8_t* input, size_t length, const uint8_t* key, const uint8_t* iv, uint8_t* output);
void aes128_cbc_decrypt(const uint8_t* input, size_t length, const uint8_t* key, const uint8_t* iv, uint8_t* output);

#endif // ENCRYPTION_H
