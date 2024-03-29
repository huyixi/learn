//
//  LandmarkRow.swift
//  Landmarks
//
//  Created by 胡一希 on 2/24/24.
//

import SwiftUI

struct LandmarkRow: View {
    var landmark: Landmark
    
    var body: some View {
        HStack {
            landmark.image
                .resizable()
                .frame(width: 50, height: 50)
            Text(landmark.name)
            Spacer()
        }
    }
}

//#Preview("Turtle Rock") {
//    
//    LandmarkRow(landmark: landmarks[0])
//}
//#Preview("Salmon") {
//    LandmarkRow(landmark: landmarks[1])
//}

#Preview {
    Group{
    LandmarkRow(landmark: landmarks[0])
    LandmarkRow(landmark: landmarks[1])
    }
}
