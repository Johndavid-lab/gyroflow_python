from setuptools import setup, find_packages
from glob import glob
import os

package_name = 'rknn_pool_yolo'

setup(
    name=package_name,
    version='0.1.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
         [os.path.join('resource', package_name)]),
        (f'share/{package_name}', ['package.xml']),
        (f'share/{package_name}/launch', glob('launch/*.launch.py')),
        (f'lib/{package_name}', ['scripts/rknn_detector']), 
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='you',
    maintainer_email='you@example.com',
    description='RKNN YOLO detector node',
    license='Apache-2.0',
    entry_points={
        'console_scripts': [
            'rknn_detector = rknn_pool_yolo.rknn_detector_node:main', 
        ],
    },
)
