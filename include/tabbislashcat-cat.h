/*
 * tabbislashcat-cat — C header
 * infant + forward movement. precision. unannounced. gone.
 * 
 * cat is the arrow. infant is the point. together, they pierce.
 */

#ifndef TABBISLASHCAT_CAT_H
#define TABBISLASHCAT_CAT_H

#include <stdint.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

/* include infant */
#include "tabbislashcat-infant.h"

/* vector structure */
typedef struct {
    uint64_t target;
    uint64_t stream_id;
    infant_token_t *tokens;
    size_t token_count;
} cat_vector_t;

/* create a new vector */
cat_vector_t *cat_vector_new(uint64_t target, uint64_t stream_id);

/* free vector and its tokens */
void cat_vector_free(cat_vector_t *vector);

/* add token to vector */
int cat_vector_add_token(cat_vector_t *vector, infant_token_t token);

/* encode vector to buffer */
/* returns: number of bytes written, or 0 on error */
size_t cat_vector_encode(const cat_vector_t *vector, uint8_t *buf, size_t len);

/* decode vector from buffer */
/* returns: number of bytes consumed, or 0 on error */
size_t cat_vector_decode(const uint8_t *buf, size_t len, cat_vector_t **vector);

/* send vector over UDP socket */
/* returns: number of bytes sent, or -1 on error */
ssize_t cat_send_udp(int sockfd, const struct sockaddr *addr, const cat_vector_t *vector);

/* receive vector from UDP socket */
/* returns: vector pointer (must be freed with cat_vector_free), or NULL on error */
cat_vector_t *cat_recv_udp(int sockfd, struct sockaddr *addr, socklen_t *addrlen);

/* reserved addresses */
#define CAT_ADDR_NULL      0x0000000000000000ULL
#define CAT_ADDR_BROADCAST 0xFFFFFFFFFFFFFFFFULL
#define CAT_ADDR_LOCALHOST 0x0000000000000001ULL

/* the void is the space where cat disappears */

#endif /* TABBISLASHCAT_CAT_H */
