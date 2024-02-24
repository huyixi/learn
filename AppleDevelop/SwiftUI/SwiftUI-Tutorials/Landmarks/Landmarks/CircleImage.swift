//
//  CircleImage.swift
//  Landmarks
//
//  Created by 胡一希 on 2/24/24.
//

import SwiftUI

struct CircleImage: View {
    var body: some View {
        Image("turtlerock")
            .clipShape(Circle())
            .overlay {
                Circle().stroke(.white,lineWidth: 4)
            }
            .shadow(radius: 10)
 
    }
}

#Preview {
    CircleImage()
}
