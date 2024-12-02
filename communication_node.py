//İHA ve uydu nesneleri arasındaki iletişimi sağlamak için ROS düğümleri oluşturacağız. 
Bu düğümler, Gazebo'da oluşturulan nesnelerden veri alacak ve bu verileri ROS topic'leri aracılığıyla iletecek.

import rospy
from std_msgs.msg import String

def send_data():
    pub = rospy.Publisher('uav_to_satellite', String, queue_size=10)
    rospy.init_node('uav_node', anonymous=True)
    rate = rospy.Rate(1)  # 1 Hz
    while not rospy.is_shutdown():
        data = "Data from UAV to Satellite"
        rospy.loginfo(data)
        pub.publish(data)
        rate.sleep()

def receive_data():
    def callback(data):
        rospy.loginfo("Received data: %s", data.data)
    rospy.init_node('satellite_node', anonymous=True)
    rospy.Subscriber('uav_to_satellite', String, callback)
    rospy.spin()

if __name__ == '__main__':
    try:
        send_data()
        receive_data()
    except rospy.ROSInterruptException:
        pass
