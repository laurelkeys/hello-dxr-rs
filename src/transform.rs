// Reference: https://github.com/microsoft/DirectXMath/blob/main/Inc/DirectXMathMatrix.inl

pub fn compute_instance_to_world_transforms(time: f32) -> [[f32; 12]; 3] {
    #[rustfmt::skip]
    let cube_to_world = {
        let (pitch, yaw, roll) = (time / 2.0, time / 3.0, time / 5.0);

        let (cp, sp) = (pitch.cos(), pitch.sin());
        let (cy, sy) = (yaw.cos(), yaw.sin());
        let (cr, sr) = (roll.cos(), roll.sin());

        // XMMatrixRotationRollPitchYaw(time / 2, time / 3, time / 5) * XMMatrixTranslation(-1.5, 2, 2)
        [
            (cr * cy + sr * sp * sy), (cr * sp * sy - sr * cy), (cp * sy), -1.5,
            (sr * cp               ), (cr * cp               ), (    -sp),  2.0,
            (sr * sp * cy - cr * sy), (sr * sy + cr * sp * cy), (cp * cy),  2.0,
        ]
    };

    #[rustfmt::skip]
    let floor_to_world = {
        // XMMatrixScaling(5, 5, 5) * XMMatrixTranslation(0, 0, 2)
        [
            5.0, 0.0, 0.0, 0.0,
            0.0, 5.0, 0.0, 0.0,
            0.0, 0.0, 5.0, 2.0,
        ]
    };

    #[rustfmt::skip]
    let mirror_to_world = {
        let ax = -1.8f32;
        let ay = time.sin() / 8.0 + 1.0;

        let (cx, sx) = (ax.cos(), ax.sin());
        let (cy, sy) = (ay.cos(), ay.sin());

        // XMMatrixRotationX(-1.8f) * XMMatrixRotationY(XMScalarSinEst(time) / 8 + 1) * XMMatrixTranslation(2, 2, 2)
        [
            ( cy     ), 0.0, ( sy     ), 2.0,
            ( sx * sy),  cx, (-sx * cy), 2.0,
            (-cx * sy),  sx, ( cx * cy), 2.0,
        ]
    };

    [cube_to_world, floor_to_world, mirror_to_world]
}
