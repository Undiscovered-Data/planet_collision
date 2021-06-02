#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <quadmath.h>

int main() {
    
    FILE *fp;
    fp = fopen("data.txt", "w");

    if (fp == NULL) {
        printf("file can't be opened\n");
        exit(1);
    }

    __float128 mass1, mass2, radius1, radius2, distance_appart, cent_mass_dist;
    __float128 grav_const = 6.6743e-11;
    __float128 speed1 = 0.0;
    __float128 speed2 = 0.0;
    __float128 accel1, accel2, a_force;
    int time_sec = 0;

    char strnum1[50];
    printf("Enter the mass of the first body\n");
    scanf("%s", strnum1);
    mass1 = strtoflt128(strnum1, NULL);

    char strnum2[50];
    printf("Enter the radius\n");
    scanf("%s", strnum2);
    radius1 = strtoflt128(strnum2, NULL);

    char strnum3[50];
    printf("Enter the mass of the second body\n");
    scanf("%s", strnum3);
    mass2 = strtoflt128(strnum2, NULL);

    char strnum4[50];
    printf("Enter the radius\n");
    scanf("%s", strnum4);
    radius2 = strtoflt128(strnum4, NULL);

    char strnum5[50];
    printf("Enter the distance appart\n");
    scanf("%s", strnum5);
    cent_mass_dist = strtoflt128(strnum5, NULL);

    distance_appart = cent_mass_dist - radius1 - radius2;

    int width = 31;
    char buf1[180];
    char buf2[180];
    char buf3[180];
    char buf4[180];
    char buf5[180];

    while (distance_appart > 0.0) {
        time_sec++;
        a_force = (grav_const * mass1 * mass2) / (cent_mass_dist * cent_mass_dist);
        accel1 = a_force / mass1;
        accel2 = a_force / mass2;
        speed1 += accel1;
        speed2 += accel2;
        cent_mass_dist = (cent_mass_dist - speed1) - speed2;
        distance_appart = (distance_appart - speed1) - speed2;

        quadmath_snprintf (buf1, sizeof buf1, "%+-#*.30Qe", width, speed1);
        quadmath_snprintf (buf2, sizeof buf2, "%+-#*.30Qe", width, speed2);
        quadmath_snprintf (buf3, sizeof buf3, "%+-#*.30Qe", width, accel1);
        quadmath_snprintf (buf4, sizeof buf4, "%+-#*.30Qe", width, accel2);
        quadmath_snprintf (buf5, sizeof buf5, "%+-#*.30Qe", width, distance_appart);

        printf ("%s, %s, %s, %s, %s\n", buf1, buf2, buf3, buf4, buf5);
        fprintf(fp, "%s, %s, %s, %s, %s\n", buf1, buf2, buf3, buf4, buf5);

    } 
    fclose(fp);
}
