package main

import "fmt"

func main() {
    var mass1, radius1, mass2, radius2, dist_appart, center_dist float64
    var grav_const float64 = 6.6743e-11
    var the_force, accel1, accel2 float64
    var speed1 float64 = 0.0
    var speed2 float64 = 0.0
    var the_seconds int = 0

    fmt.Println("Enter a mass1")
    fmt.Scanf("%f", &mass1)
    fmt.Println("Enter radius1")
    fmt.Scanf("%f", &radius1)
    fmt.Println("Enter a mass2")
    fmt.Scanf("%f", &mass2)
    fmt.Println("Enter radius2")
    fmt.Scanf("%f", &radius2)
    fmt.Println("Enter the distance")
    fmt.Scanf("%f", &center_dist)

    dist_appart = (center_dist - radius1) - radius2

    for dist_appart > 0 {
        the_force = (grav_const * mass1 * mass2) / (center_dist * center_dist)
        accel1 = the_force / mass1
        accel2 = the_force / mass2
        speed1 = speed1 + accel1
        speed2 = speed2 + accel2
        dist_appart = (dist_appart - speed1) - speed2
        the_seconds++

        fmt.Println(accel1, speed1, accel2, speed2, dist_appart)
    }
}
