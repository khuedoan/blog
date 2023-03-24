import { AiFillMail, AiFillGithub, AiFillLinkedin, AiOutlineTwitter } from "react-icons/ai";
import { IconBaseProps } from "react-icons/lib";

const social = [
  {
    name: 'Mail',
    href: 'mailto:mail@khuedoan.com',
    icon: (props: IconBaseProps) => <AiFillMail {...props}/>,
  },
  {
    name: 'GitHub',
    href: 'https://github.com/khuedoan',
    icon: (props: IconBaseProps) => <AiFillGithub {...props}/>,
  },
  {
    name: 'LinkedIn',
    href: 'https://linkedin.com/in/khuedoan',
    icon: (props: IconBaseProps) => <AiFillLinkedin {...props}/>,
  },
  {
    name: 'Twitter',
    href: 'https://twitter.com/KhueDoanID',
    icon: (props: IconBaseProps) => <AiOutlineTwitter {...props}/>,
  },
]

export default function SocialIcons() {
  return (
    <div className="mt-10 flex justify-center space-x-5">
      {social.map((item) => (
        <a key={item.name} href={item.href} className="text-gray-400 hover:text-gray-500">
          <span className="sr-only">{item.name}</span>
          <item.icon className="h-6 w-6" aria-hidden="true" />
        </a>
      ))}
    </div>
  )
}
