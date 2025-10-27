#[cfg(test)]
mod tests {
    use crate::vec2::*;

    #[test]
    fn add() {
        let player = vec2(2, 3);
        let enemy = vec2(3, 1);

        assert_eq!(player + enemy, vec2(5, 4));
        assert_eq!(player - enemy, vec2(-1, 2));
        assert_eq!((player * enemy) / enemy, vec2(2, 3));

        assert!(player != enemy);
    }

    #[test]
    fn mul() {
        let mut player = vec2(3.176, 1.5);
        let enemy = vec2(0.45, 7.3);

        player *= enemy;
        assert!(player.x > consts::f32::ZERO.x);

        player /= -1.0;
        assert!(player.y < consts::f32::ZERO.y);
    }
}
