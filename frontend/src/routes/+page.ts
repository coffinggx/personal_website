import type {PageLoad} from "./$types';

export default load: PageLoad =async () =>{
  const res= await fetch("https://:8080/get");
  const data = await res.json();
  return data;
}
