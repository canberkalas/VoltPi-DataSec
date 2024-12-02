docker run -it ros-gazebo
source /opt/ros/melodic/setup.bash
roslaunch gazebo_ros empty_world.launch
rosrun my_package communication_node.py
